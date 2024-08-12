<p align="center">
  <img src="https://github.com/tryanything-ai/anything/blob/main/apps/web/public/magic_3po.webp" height="300" alt="Anything" />
</p>
<p align="center">
  <em>If Posthog built Zapier. Feature complete, Fullstack, AI Automation framework written in Rust made for users.</em>
</p>

<p align="center">
<a href="https://www.tryanything.xyz/">🔗 Main site</a>
<span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
<a href="https://discord.gg/95pNMNGW7c">💬 Discord</a>
<span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
<a href="https://www.loom.com/share/c71dc4d5a07c4424b3f6d5bbe218549f?sid=6eb7eb8c-4acd-44e2-ae4e-d9563d1a376a">💻 Demo Video</a>

# Anything AI

![Anything UI](https://raw.githubusercontent.com/tryanything-ai/anything/main/assets/anything_dnd_sept_11.gif)

### Make Anything happen

![Anything UI](https://raw.githubusercontent.com/tryanything-ai/anything/main/assets/anything_sept_11.gif)

### 🔧 Setting Up

```bash
git clone https://github.com/tryanything-ai/anything.git
pnpm dev
```

## Systems

### Workflow Rest API using [Axum](https://github.com/tokio-rs/axum)

- [x] Workflow CRUD API via [Postgrest](https://github.com/supabase-community/postgrest-rs) over [Supabase](https://supabase.com/)
- [x] Workflow Versions Management
- [x] Workflow Publishing / Active Management

### Durable Workflow Processing

- [x] Worfklow Traversal to Task Planning
- [x] Action Configuration Bundling with [Tera](https://keats.github.io/tera/docs/)
- [x] Task Queue CRUD via [Postgrest](https://github.com/supabase-community/postgrest-rs) over [Supabase](https://supabase.com/)
- [x] Task Queue Processing System
- [x] Trigger Management System
  - [x] Cron Triggers
  - [ ] Webhook Triggers
  - [ ] Polling Triggers

### Authentication and Authorization

- [x] User Mangement via [Supabase](https://supabase.com/)
- [x] User Oauth Integration Mangement
- [x] Team Auth and Billing System using [BaseJump](https://usebasejump.com/) open source template based on Postgres [Row Level Security](https://www.postgresql.org/docs/current/ddl-rowsecurity.html)

### Workflow Studio

- [x] Drag and Drop Editor via [ReactFlow](https://reactflow.dev/)
- [x] Action Configuration via [json-schema-form](https://github.com/remoteoss/json-schema-form)
- [x] UI via [TailwindCSS](https://tailwindcss.com/) and [ShadCN](https://ui.shadcn.com/)
- [x] Workflow Testing Management
- [ ] Single Action Testing
- [ ] Action Templates Management
- [ ] Workflow Templates Management

### Template Marketplace

- [x] User Profiles
- [x] Publish Workflow Templates
- [x] Publish Action Templates

### 💌 Feedback

Love Anything? Give us a star ⭐️!

# Architecture Goals

- An open automation tool that allows for maximum creativity and extensibility without sacrificing <ins>understandability</ins>.
- An architecture that lends itself towards the <ins>incremental adoption of new AI</ins> no matter which "shape" it takes
- An architecture that is focused on skating towards the puck of <ins>self authoring</ins> by storing state, logs, events etc in human centric, sovereign mediums easily understood and created by low cost LLM's.

##### Extensibility without sacrificing understandability

- Each Action Type is defined by a Plugin.
- Think of Plugins the same as in VSCode but they execute tasks.
- You can <ins>author your own plugins</ins> or grab them from the community.
- Plugins are WASM and can be written in any language that can compile to it.

#### User Interface Opinions

- Designed to be <ins>self describing</ins> so at first glance flows describe what they do more than "how" they do it
- Configuring is done through {{templating}} arguments with access to previous results, .env, system constants etc

### Core Team:

Carl Lippert: [Twitter](https://twitter.com/carllippert)
