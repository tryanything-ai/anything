# 🚀 Anything [Alpha Pre Release]

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

### 🛠 Technologies Used

- Rust: Ensuring robust and lightning-fast operations.
- Deno: Lets you use custom JS, TS, Wasm etc. 
- Tauri: Local apps written in Rust.

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

- All flows are just files that can live in Version Control
- Inspired by NextJS routing a flow just lives at ~/Documents/Flows/{Flow Name} in your computer

[Go to an example flow definition](https://github.com/tryanything-ai/anything/tree/main/assets/examples)

### 🤖 Roadmap

##### Core [ Free ]
- [x] Embeded Sqlite DB
- [X] WYSIWYG Editor
- [x] Event System
- [ ] Flow Version Control
- [ ] Custom Extensions ( Like in VSCode )
- [ ] Sqlite Vectors 
- [ ] WASM Interpreter
- [ ] Python Interpreter
- [ ] Local AI Models
- [ ] Developer Documentation

##### Ecosystem [ Free ]
- [x] Template Marketplace @ [www.tryanything.yz](https://www.tryanything.xyz/)
- [ ] Action Marketplace
- [ ] Extensions Marketplace

##### Business [ Paid ]
- [ ] Integration with popular apps and services

### 💌 Feedback

Love Anything? Give us a star ⭐️!

### Contact

Carl Lippert on [Twitter](https://twitter.com/carllippert)
