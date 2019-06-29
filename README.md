# Discord Jebait

This server will inspect the **User-Agent** header of the HTTP request and serve a fake image for the Discord Bot (and also for Macs, because Discord servers are checking the URL with a Mac apparently as well).

Opening the image in the browser reveals the real contents.

Requires 2 files in the working directory: `praying-jesus.jpg` and `praying-jesus.webm`.

The `.jpg` gets displayed for the Discord Bot and the `.webm` for everyone else.
