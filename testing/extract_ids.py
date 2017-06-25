#!/usr/bin/env python3
"""
This is a little helper script that provides a means to extract a list of MBIDs
from a MusicBrainz database dump. You can download a copy of `mbdump.tar.bz2`
from here: https://musicbrainz.org/doc/MusicBrainz_Database/Download#Download

Extract the file to a .tar file. (Around 7.5 GB)
You can use `lbzip2 -d [filename]` to speed things up.
"""

import os
import random
import sys
import tarfile

NLIMIT=100000

README = """The contents of this file are a random sample of MBIDs of the MusicBrainz entities.
Source of the original dataset:
    https://musicbrainz.org/doc/MusicBrainz_Database
As the relevant data was released under the CC0 license, which you can find at:
    https://creativecommons.org/publicdomain/zero/1.0/
the list of MBIDs is also licensed as CC0.
"""

def extract(filename):
    archive = tarfile.open(filename, "r")
    if not os.path.exists("mbids"):
        os.mkdir("mbids")

    with open("mbids/README", "w") as readme:
        readme.write(README)

    entities = (("Area", "mbdump/area"),
                ("Artist", "mbdump/artist"),
                #("ArtistCredit", "mbdump/artist_credit"),
                ("Event", "mbdump/event"),
                ("Label", "mbdump/label"),
                #("Medium", "mbdump/medium"),
                ("Place", "mbdump/place"),
                ("Recording", "mbdump/recording"),
                ("Release", "mbdump/release"),
                ("ReleaseGroup", "mbdump/release_group"),
                ("Series", "mbdump/series"),
                ("Track", "mbdump/track"),
                ("URL", "mbdump/url"),
                ("Work", "mbdump/work"))

    for entity_name, entity_member in entities:
        print("Extracting entitiy: {}".format(entity_name))
        source = archive.extractfile(entity_member)
        with open("mbids/{}".format(entity_name), "w") as target:
            all_lines = source.readlines()
            lines = random.sample(all_lines, min(NLIMIT, len(all_lines)))
            for line in lines:
                target.write(line.decode("utf8").split("\t")[1] + "\n")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("usage: extract_ids.py path/to/mbdump.tar")
    else:
        extract(sys.argv[1])

