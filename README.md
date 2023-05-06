# zettai-ryouiki

> "Zettai ryōiki refers to the area of bare skin in the gap between overknee socks and a miniskirt."

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
[![forthebadge](https://forthebadge.com/images/badges/60-percent-of-the-time-works-every-time.svg)](https://forthebadge.com)
[![forthebadge](https://forthebadge.com/images/badges/built-with-swag.svg)](https://forthebadge.com)

## About

Pixiv downloader written in Rust. Uses async code and should be relatively fast, although it might struggle with a large amount of images.

## TODO:

- [x] Download all illustrations images per id
- [x] Add support for user/author illustrations downloads
- [x] Support for cookie auth for R18/sensitive images
- [x] Download Pixiv Ugoira animated illustrations
- [ ] Try not to get rate limited (lol)
- [ ] Animate Ugoira images (maybe)
- [ ] Retry on error
- [ ] Add support for bookmarks
- [ ] Limit how many images to download

## Contributing

1.  Fork the repo on GitHub
2.  Clone the project to your machine
3.  Commit changes to your branch
4.  Push your work to your fork
5.  Submit a Pull request so that I can review your changes

NOTE: Be sure to merge the latest from "upstream" before making a pull request!

## License

Copyright 2021 pxseu

Licensed under the Mozilla Public License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. \
You may obtain a copy of the License at:

> https://www.mozilla.org/en-US/MPL/2.0/

A copy of the license is available in the repository's [LICENSE](./LICENSE) file.
