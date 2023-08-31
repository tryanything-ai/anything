# 🚀 Anything

✨ The next-generation local automation tool: Think Zapier, but on your Mac, supercharged with AI!

![Anything UI](https://raw.githubusercontent.com/tryanything-ai/anything/main/docs/anything_aug_10.gif)


### 🌟 Welcome to Anything - the future of local automation! 

If you've been seeking a tool that blends the simplicity of Zapier, the flexibility of AI, and the power of a code editor, look no further.

### 💡 Why Anything?

- Local Power: Why get locked into some SaaS when you've got Apple Silicon under the sheets?
- AI Integration: Boost your workflows with integrated AI models.
- WYSIWYG Designer: Visualize your automation workflows like never before.
- 100% Open Source: Freedom to modify, integrate, and extend.

### 🛠 Technologies Used

- React: For our intuitive and seamless UI.
- Rust: Ensuring robust and lightning-fast operations.
- Tauri: Local apps that aren't a whole chrome browser. 

### 🤝 Contribute

We're on the lookout for passionate developers to build with. Dive into our code, bring your crazy ideas, and let's build. 

But first. 

Check out our [contributors guide](https://github.com/tryanything-ai/anything/tree/main/.github/CONTRIBUTING.md).

### 🔧 Setting Up
```bash
git clone https://github.com/tryanything-ai/anything.git
cd tauri
pnpm i
pnpm start 
```

### Flows are defined as TOML Files
- All flows are just files
- Inspired by NextJS routing a flow just lives at ~/Documents/Flows/{Flow Name} in your computer

[Go to an example flow definition](https://github.com/tryanything-ai/anything/tree/main/docs/examples)


### 🤖 Roadmap
- [x] [Monaco](https://github.com/suren-atoyan/monaco-react) Editor ( same as vscode )
- [x] Embeded Sqlite DB
- [ ] Event System ( in progress )
- [ ] Sqlite Vectors - [TinyVector](https://github.com/m1guelpf/tinyvector) in Rust
- [ ] [Rustformers](https://github.com/rustformers/llm) for local AI models 
- [ ] Python Interpreter
- [ ] Javascript Runtime w/ [Deno](https://github.com/denoland/deno)
- [ ] Integration with popular local apps and services

Share your ideas!

### 💌 Feedback
Love Anything? Give us a star ⭐️! 

### Contact: 
Carl Lippert on [Twitter](https://twitter.com/carllippert)

