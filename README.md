# 🚀 Anything [Alpha Pre Release]

✨ Think Zapier, but on your Mac, completely focused on AI!

## Don't hire your next employee. Build them!

### Drag it and drop it

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

### 🛠 Technologies Used

- React: For our intuitive and seamless UI.
- Rust: Ensuring robust and lightning-fast operations.
- Deno: Lets you use custom JS, TS, Wasm etc. 
- Tauri: Local apps that aren't a whole chrome browser.

### Where we're going

![Agent Library](https://raw.githubusercontent.com/tryanything-ai/anything/main/assets/agent_library.png)

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

### Flows are defined as TOML Files

- All flows are just files
- Inspired by NextJS routing a flow just lives at ~/Documents/Flows/{Flow Name} in your computer

[Go to an example flow definition](https://github.com/tryanything-ai/anything/tree/main/assets/examples)

### 🤖 Roadmap

- [x] [Monaco](https://github.com/suren-atoyan/monaco-react) Editor ( same as vscode )
- [x] Embeded Sqlite DB
- [x] WASM Interpreter via [Deno](https://github.com/denoland/deno)
- [x] Event System
- [x] Allow Custom Rust Plugins for Powerful Extensions
- [x] Template Marketplace @ [www.tryanything.yz](https://www.tryanything.xyz/)
- [ ] Sqlite Vectors - [TinyVector](https://github.com/m1guelpf/tinyvector) in Rust
- [ ] [Rustformers](https://github.com/rustformers/llm) for local AI models
- [ ] Python Interpreter via Starlark
- [ ] Integration with popular local apps and services

Share your ideas!

### 💌 Feedback

Love Anything? Give us a star ⭐️!

### Contact

Carl Lippert on [Twitter](https://twitter.com/carllippert)
