<p align="center">
  <img src="https://github.com/tryanything-ai/anything/blob/main/apps/web/public/3og.svg" height="300" alt="Anything" />
</p>
<p align="center">
  <em>Rebuilding Zapier in Rust to make Local AI do way more than chat</em>
</p>

<p align="center">
<a href="https://www.tryanything.xyz/">🔗 Main site</a>
<span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
<a href="https://discord.gg/95pNMNGW7c">💬 Discord</a>
</p>

# Anything

✨ Have AI do work for you!

## Don't hire your next employee. Build them!

![Anything UI](https://raw.githubusercontent.com/tryanything-ai/anything/main/assets/anything_dnd_sept_11.gif)

### Make Anything happen

![Anything UI](https://raw.githubusercontent.com/tryanything-ai/anything/main/assets/anything_sept_11.gif)

### 🌟 Welcome to Anything - the future of local automation!

Looking for a tool that blends the simplicity of Zapier, the power of AI and the the ability to be self hosted?

Look no further.

Anything will be the first tool you go to grab when your dreaming of putting AI to work for you.

### 💡 Why Anything?

- Local Power: Why get locked into some SaaS when you've got Apple Silicon under the sheets?
- AI Integration: Boost your workflows with integrated AI models.
- WYSIWYG Designer: Visualize your automation workflows like never before.
- 100% Open Source: Freedom to modify, integrate, and extend.

### 🤝 Contribute

We're on the lookout for passionate developers to build with. Dive into our code, bring your crazy ideas, and let's build.

But first.

Check out our [contributors guide](https://github.com/tryanything-ai/anything/tree/main/.github/CONTRIBUTING.md).

### 🔧 Setting Up

```bash
git clone https://github.com/tryanything-ai/anything.git
pnpm quick
```

### 💻 Developing
```bash
pnpm dev
```
-> runs all shared packages in watch mode. 
-> runs tauri && website && docs (soon)

For a lighter weight experience just run dev on the app you are working on in /apps and remember to build or dev the packages it consumes if your working on them

Repo structure based on turborepo tailwind template
-> https://github.com/vercel/turbo/tree/main/examples/with-tailwind


[Go to an example flow definition](https://github.com/tryanything-ai/anything/tree/main/assets/examples)

### 🤖 Roadmap

##### Core 
- [x] Embeded Sqlite DB
- [x] WYSIWYG Editor
- [x] Event System
- [ ] Custom Extensions ( Like in VSCode )
- [ ] Sqlite Vectors 
- [ ] Deno Extension
- [ ] Python Extension
- [ ] Local AI Extension
- [ ] Developer Documentation
- [ ] Docker Version for Cloud Hosting 24/7
- [ ] Flow Version Control ( Stages, Semantic Versioning, etc)

##### Ecosystem 
- [x] Template Marketplace @ [www.tryanything.yz](https://www.tryanything.xyz/templates)
- [ ] Action Marketplace
- [ ] Extensions Marketplace

##### Business [ Paid ]
- [ ] Integration with popular apps and services
- [ ] Webhook "Mailbox" to persist incoming requests for later processing
- [ ] Enterprise Hosting Plans

### 💌 Feedback

Love Anything? Give us a star ⭐️!

### Architecture

#### Core Goals
- An open automation tool that allows for maximum creativity and extensibility without sacrificing __understandability__. 
- An architecture that lends itself towards the __incremental adoption of new AI__ no matter which "shape" it takes
- An architecture that is focused on skating towards the puck of __self authoring__ by storing state, logs, events etc in human centric, sovereign mediums easily understood and created by low cost local LLM's. 

##### Application state is __Simple and Understandable__
- State of flows is just a __file__ that can be kept in __version control__
- State is __File First__ which means it can be edited from an IDE or the Application with equal support.
- Triggers, Actions, and Flows are portable and fully encapsulated.
- File and Folder names are __Human Centric__ following similar design patters as NextJS routing.

##### Application does not require docker
- Makes it easy to adopt like normal apps
- Makes it so it can run all day even on low powered devices

#### Event Processing focuses on simple vs fast. Buts its still fast. 
- Events are stored in an event queue based on SQLite
- Starting and stopping at any point is easy.
- Past state is all visible making it easy to debug failure

##### Extensibility without sacrificing understandability
- Each Action is defined by an Extension.
- Think of Extensions the same as in VSCode but they process events.
- You only download the extensions you need protecting the project from "package bloat"
- You can author your own extensions or grab them from the community
- Extensions are written in Rust so you can also write them in other interpretted languages like Python or Typescript

##### Extension Interface
- defines an "execution" function to process events
- defines a "validation" function for validating user configuration dynamically. This also helps LLM's write config with high certainty from the feedback
- defines an "action" the node a user see's, the SVG, the name, default arguments, etc
- has access to event system and full flow definition it exists inside of to allow for arbitrary complexity of loop and decision nodes that are a common problem point in automation tools. 

#### User Interface
- Designed to be __self describing__ so at first glance flows describe what they do more than "how" they do it.
- Configuring is done through {{templating}} arguments with access to previous results, .env, system constants etc.

#### Logs
- Everything logged into Open Telemetry
- Makes even application bugs accessible as a single layer to future self authoring AI so it can tell if a problem is form the software or the user
- Makes easy to adopt into different clouds and organizations


### Contact
Carl Lippert on [Twitter](https://twitter.com/carllippert)
