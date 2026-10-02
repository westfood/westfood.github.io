# Rudolf Kreibich — personal presentation

A static CV website built with plain HTML and CSS. The publishable site lives
in `docs/`; there are no frontend
dependencies or build steps.

## Local preview

Install [uv](https://docs.astral.sh/uv/getting-started/installation/) and make sure
Python 3 is available, then run:

```sh
./run.sh
```

Open <http://127.0.0.1:8000>. Stop the server with `Ctrl+C`.

To choose a different port:

```sh
./run.sh 8080
# or
PORT=8080 ./run.sh
```

The script works from any current directory and serves only `docs/`. It uses
`uv` to run Python's standard-library HTTP server, bound to localhost.

On Rudolf's Mac, `run.sh` loads the Building storage environment and verifies
the external volume before running `uv`. Mount `/Volumes/Building` before
starting. Keep generated screenshots, reports, logs, and other preview artifacts
under `/Volumes/Building/rudolf/artifacts/westfood-cv`, with scratch files
under `/Volumes/Building/rudolf/tmp`; the website source stays in this checkout.

## Files

- `docs/index.html`: page content and structure.
- `docs/styles.css`: responsive styling.
- `docs/assets/`: website assets and downloadable CV.
- `run.sh`: local preview server.

The page follows the CV's paper, red and blue palette, condensed typography and
portrait. All descriptions are visible directly on the page; no JavaScript
or external service is needed to render the site. The site is in English and
includes language metadata and a canonical URL. The three-page PDF is
downloadable, with email and telephone removed from the website copy. “Get in
touch” opens LinkedIn's message composer addressed to Rudolf in a new tab;
visitors need to sign in and LinkedIn's messaging permissions apply.
Anton and Rajdhani are self-hosted WOFF2 fonts; their SIL Open Font License files
are included in `docs/assets/fonts/`. Typography was visually identified because
the supplied PDF outlines its text. Red card backgrounds use a slightly darker
shade so white body text remains readable.

## Publishing

GitHub Pages is configured to publish **`master` → `/docs`** at
[iam.itchy.cz](https://iam.itchy.cz). Preserve `docs/CNAME` (`iam.itchy.cz`),
`docs/.nojekyll`, and the matching root `CNAME`.

Preview and review changes locally first. Pushing changes to `master` can update
the public site automatically; publish only after the local preview is approved.
The `cv-local-preview` branch is for local review.
