#!/usr/bin/env python3
import argparse
import os
import random
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

if __name__ == "__main__":
    all_entities = "Area,Artist,Event,Label,Place,Recording,ReleaseGroup,Series,Track,URL,Work"

    parser = argparse.ArgumentParser()
    p_subs = parser.add_subparsers(dest="action")
    p_generate = p_subs.add_parser("generate")
    p_generate.add_argument("-e", "--entities", help="The entities for which tests are to be generated, comma separated without spaces.", default=all_entities)
    p_generate.add_argument("-n", "--num", default=25, help="Number of test cases per entity.")

    p_run = p_subs.add_parser("run")

    args = parser.parse_args()
    if args.action == "generate":
        entities = args.entities.split(",")
        for e in entities:
            if not e in all_entities:
                print("Error: Unknown entity: " + e)
                sys.exit(2)

        generate_tests(entities=entities, num=int(args.num))
    elif args.action == "run":
        print("Test running not implemented yet.")
    else:
        parser.print_help()
        sys.exit(2)
