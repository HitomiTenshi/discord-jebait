# Discord Jebait

This server will inspect the **User-Agent** header of the HTTP request and serve a fake image for the Discord Bot.

Opening the image in the browser reveals the real contents.

Requires 2 files in the working directory: `praying-jesus.jpg` and `praying-jesus.webm`.

The `.jpg` gets displayed for the Discord Bot and the `.webm` for everyone else.

Visit http://127.0.0.1:54299/praying-jesus.jpg when running the server. To see the effect change your User-Agent string to `Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)` and reload the page.

## Live examples

- Greentext jebait: https://dekinai.moe/N9vQN.jpg
- Gachi jebauit (**NSFW**): https://omegalul.dekinai.moe/praying-jesus.jpg

Either paste those examples to someone in Discord (or in your own Discord server), or change your User-Agent to the string mentioned above in your browser and load / refresh the page.
