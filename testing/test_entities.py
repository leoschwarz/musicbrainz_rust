#!/usr/bin/env python3
import argparse
import datetime
import os
import random
import shutil
import subprocess
import sys

MBIDS_FILE="https://leoschwarz.com/git-assets/musicbrainz_rust/mbids.tar.gz"

def fetch_mbids(entity, num):
    if not os.path.exists("mbids"):
        print("Fetching MBIDs index.")
        subprocess.Popen(["wget", MBIDS_FILE]).wait()
        subprocess.Popen(["tar", "xzf", "mbids.tar.gz"]).wait()

    with open("mbids/"+entity, "r") as f:
        return [line.strip() for line in random.sample(f.readlines(), num)]

TEST_PREAMBLE = """
#[macro_use]
extern crate log;
extern crate musicbrainz;
extern crate pretty_env_logger;
extern crate reqwest_mock;

use std::str::FromStr;
use musicbrainz::client::{Client, ClientConfig};
use musicbrainz::entities::*;
use musicbrainz::ClientError;
use reqwest_mock::GenericClient as HttpClient;

#[test]
fn run_tests() {
    pretty_env_logger::init().unwrap();

    let mut client = Client::with_http_client(ClientConfig {
        user_agent: "musicbrainz_rust/testing (mail@leoschwarz.com)".to_owned(),
        max_retries: 5,
    }, HttpClient::replay_dir("replay/test/test"));

    let mut failures = 0;
"""

TEST_TEMPLATE = """
    let mbid = Mbid::from_str("$MBID").unwrap();
    let res = client.get_by_mbid::<$ENTITY>(&mbid);
    match res {
        Ok(_) => {info!("Test $ENTITY-$MBID successful.");}
        Err(e) => {
            info!("Test $ENTITY-$MBID failed, error: {:?}", e);
            failures += 1;
        }
    }
"""

TEST_END = """
    println!("{} failures", failures);
    assert!(failures == 0);
}
"""

def generate_tests(entities, num):
    print("Generating {} tests each for entities: {}".format(num, ", ".join(entities)))

    code = []
    code.append(TEST_PREAMBLE)

    for entity in entities:
        mbids = fetch_mbids(entity, num)
        for mbid in mbids:
            mbid_min = mbid.replace("-", "")
            test = TEST_TEMPLATE.replace("$TESTNAME", "{}_{}".format(entity, mbid_min)) \
                                .replace("$MBID", mbid) \
                                .replace("$ENTITY", entity)
            code.append(test)

    code.append(TEST_END)

    with open("tests.rs", "w") as f:
        f.write("".join(code))

def run_tests(source_file, target, keep):
    def target_path(t):
        dirname = os.path.dirname(os.path.abspath(__file__))
        return os.path.join(dirname, "..", "tests", t)

    # Check the source file.
    if os.path.exists(source_file):
        print("Running test file: {}".format(source_file))
    else:
        print("Error: Source file `{}` doesn't exist.".format(source_file))

    # Determine the target path if needed.
    if target is None:
        now = datetime.datetime.now()
        template = now.strftime("entities_%Y_%m_%d_{}.rs")
        i=1
        target = template.format(i)
        while os.path.exists(target_path(target)):
            i += 1
            target = template.format(i)

    # Copy the source file to the target path.
    shutil.copyfile(source_file, target_path(target))

    # Run the tests.
    env = os.environ.copy()
    env["RUST_LOG"] = "{}=info".format(target.split(".")[0])
    subprocess.Popen(["cargo", "test"], env=env).wait()

    # Delete the file unless we are supposed.
    if not keep:
        os.remove(target_path(target))
        print("Test file {} was removed from the tests directory.".format(target))


if __name__ == "__main__":
    all_entities = "Area,Artist,Event,Label,Place,Recording,ReleaseGroup,Series,Track,URL,Work"

    parser = argparse.ArgumentParser()
    p_subs = parser.add_subparsers(dest="action")

    p_generate = p_subs.add_parser("generate")
    p_generate.add_argument(
        "-e", "--entities",
        help="The entities for which tests are to be generated, comma separated without spaces.",
        default=all_entities)
    p_generate.add_argument("-n", "--num", default=25, help="Number of test cases per entity.")

    p_run = p_subs.add_parser(
        "run",
        help="Run the test file (by default it has to be the `tests.rs` file in the CWD but this can be changed with `--source`).")
    p_run.add_argument(
        "-k", "--keep", default=False, action="store_true",
        help="If specified the test file will remain in the project's `tests` directory.")
    p_run.add_argument(
        "-s", "--source", default="tests.rs",
        help="If wished you can opt to run a different tests file than `tests.rs` by specifying its name here.")
    p_run.add_argument(
        "-t", "--target", default=None,
        help="Target name of the test case in the `tests` directory.")

    args = parser.parse_args()
    if args.action == "generate":
        entities = args.entities.split(",")
        for e in entities:
            if not e in all_entities:
                print("Error: Unknown entity: " + e)
                sys.exit(2)

        generate_tests(entities=entities, num=int(args.num))
    elif args.action == "run":
        run_tests(source_file=args.source, target=args.target, keep=args.keep)
    else:
        parser.print_help()
        sys.exit(2)
