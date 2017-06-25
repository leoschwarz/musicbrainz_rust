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
extern crate musicbrainz;
extern crate reqwest_mock;

use std::str::FromStr;
use musicbrainz::client::ClientConfig;
use musicbrainz::entities::*;
use reqwest_mock::DirectClient;

type Client = musicbrainz::client::Client<DirectClient>;

fn get_client() -> Client {
    let config = ClientConfig {
        user_agent: "musicbrainz_rust/testing (mail@leoschwarz.com)".to_owned(),
    };
    Client::new(config).unwrap()
}

"""

TEST_TEMPLATE = """
#[test]
fn read_$TESTNAME() {
    let mbid = Mbid::from_str("$MBID").unwrap();
    let client = get_client();
    client.get_by_mbid::<$ENTITY>(&mbid).unwrap();
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

    with open("tests.rs", "w") as f:
        f.write("".join(code))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    p_subs = parser.add_subparsers(dest="action")
    p_generate = p_subs.add_parser("generate")
    #p_generate.add_argument("-e", "--entitiy", help="The entities for which tests are to be genarated.")
    p_generate.add_argument("-n", "--num", default=25, help="Number of test cases per entity.")

    p_run = p_subs.add_parser("run")

    args = parser.parse_args()
    if args.action == "generate":
        # TODO make configurable
        #entities = ["Area", "Artist", "Event", "Label", "Recording", "Release", "ReleaseGroup"]
        entities = ["Area", "Artist", "Event", "Release", "ReleaseGroup"]
        generate_tests(entities=entities, num=int(args.num))
    elif args.action == "run":
        print("Test running not implemented yet.")
    else:
        parser.print_help()
        sys.exit(2)
