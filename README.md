<p align="center">
  <img src="https://github.com/tryanything-ai/anything/blob/main/apps/web/public/magic_3po.webp" height="300" alt="Anything" />
</p>
<p align="center">
  <em>Rebuilding Zapier in Rust to make Local AI do way more than chat</em>
</p>

<p align="center">
<a href="https://www.tryanything.xyz/">🔗 Main site</a>
<span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
<a href="https://discord.gg/95pNMNGW7c">💬 Discord</a>
<span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
<a href="https://www.loom.com/share/c71dc4d5a07c4424b3f6d5bbe218549f?sid=6eb7eb8c-4acd-44e2-ae4e-d9563d1a376a">💻 Demo Video</a>
<span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
<a href="https://github.com/tryanything-ai/anything/releases">💿 Download App</a>
</p>

# Anything


![Anything UI](https://raw.githubusercontent.com/tryanything-ai/anything/main/assets/anything_dnd_sept_11.gif)

### Make Anything happen

![Anything UI](https://raw.githubusercontent.com/tryanything-ai/anything/main/assets/anything_sept_11.gif)


### 🔧 Setting Up
```bash
git clone https://github.com/tryanything-ai/anything.git
pnpm dev
```



### 🤖 Roadmap

##### Core 
- [x] Embeded Sqlite DB
- [x] WYSIWYG Editor
- [x] Event System
- [x] Cron Trigger
- [x] HTTP Extension
- [x] CLI Extension
- [ ] Custom Extensions ( probably WASM )
- [ ] Sqlite Vectors 
- [ ] Deno Extension
- [ ] Python Extension
- [ ] Local AI Extension
- [ ] Developer Documentation
- [ ] Docker Version for Cloud Self Hosting 24/7
- [ ] Flow Version Control ( Stages, Semantic Versioning, etc)

##### Ecosystem 
- [ ] Template Marketplace
- [ ] Action Marketplace
- [ ] Trigger Marketplace
- [ ] Extension Marketplace


### 💌 Feedback

Love Anything? Give us a star ⭐️!

# Architecture Goals

- An open automation tool that allows for maximum creativity and extensibility without sacrificing <ins>understandability</ins>. 
- An architecture that lends itself towards the <ins>incremental adoption of new AI</ins> no matter which "shape" it takes
- An architecture that is focused on skating towards the puck of <ins>self authoring</ins> by storing state, logs, events etc in human centric, sovereign mediums easily understood and created by low cost local LLM's. 

##### Application state is <ins>Simple and Understandable</ins>
- State of flows is just a <ins>file<ins> that can be kept in <ins>version control</ins>
- State is <ins>File First</ins> which means it can be edited from an IDE or the Application with equal support
- Triggers, Actions, and Flows are portable and fully encapsulated.
- File and Folder names are <ins>Human Centric</ins> following similar design patters as NextJS routing

##### Application does not require docker
- Makes it easy to adopt like normal apps
- Makes it so it can run all day even on low powered device

#### Event Processing focuses on simple vs fast. Buts its still fast. 
- Events are stored in an event queue based on SQLite
- Starting and stopping at any point is easy.
- Past state is all visible making it easy to debug failure

##### Extensibility without sacrificing understandability
- Each Action is defined by an Extension.
- Think of Extensions the same as in VSCode but they execute events.
- You only download the extensions you need protecting the project from "package bloat"
- You can <ins>author your own extensions</ins> or grab them from the community
- Extensions are written in Rust so you can also write them in other interpreted languages like Python or Typescript

##### Extension Interface
- defines an "execution" function to process events
- defines a "validation" function for validating user configuration. This allows for "deterministic magic" preventing hallucinating humans or LLM's from writing bad configurations
- defines an "action" the node a user see's, the SVG, the name, default arguments, etc
- has access to event system and full flow definition it exists inside of to allow for arbitrary complexity of loop and decision nodes that are a common problem point in automation tools

#### User Interface
- Designed to be <ins>self describing</ins> so at first glance flows describe what they do more than "how" they do it
- Configuring is done through {{templating}} arguments with access to previous results, .env, system constants etc

#### Logs
- Everything logged into Open Telemetry
- Makes even application bugs accessible as a single layer to future self authoring AI so it can tell if a problem is form the software or the user
- Makes easy to adopt into different clouds and organizations


### Core Team:

Carl Lippert: [Twitter](https://twitter.com/carllippert)
