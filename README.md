# Remote Image Viewer (RIV)

Image viewing website for easily sharing and categorizing images with minimal storage requirements.

The app saves links to images hosted on other sites, since they were doing that anyways, might as well use them.

### Running

To run the app you will first neec to start the database with:

(you might need to use sudo when on linux)
```sh
docker compose -f compose.yml build
```

If you have build the db before you can also run:
```sh
docker compose -f compose.yml up -d
```

Make sure to close the docker once you're done with:
```sh
docker compose down
```

Run the actual app with:
```sh
dx serve
```

(Running the app with docker coming soon:tm:)
