# Description
In addition to the unit tests included in the modules in the `src` directory, we want to test the library against as much real world data as possible.
Hence in this directory we find utilities to generate tests with MusicBrainz data.

As of now the goal is to only find entities this way where parsing completely fails, i.e. returning an `Err` instead of an `Ok(_)`. We simply fetch entities from the MusicBrainz database and put it through our code and observe what happens.

# Status
Nothing of this automated testing strategy is implemented as of now.

