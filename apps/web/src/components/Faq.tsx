const faqs = [
    {
        id: 1,
        question: "If its free how do you make money?",
        answer:
            "We will charge large businesses for the additional features they need.",
    },
    {
        id: 2,
        question: "Whats the difference between a plugin and an extension?",
        answer:
            "Nothing. Every app has to choose what to name things. We chose to use the word extension. ",
    },
    {
        id: 3,
        question: "How are actions and extensions related to eachother in Anything?",
        answer:
            "Each type of action has an extension that does the work. The extension is the code that runs the action. In some ways an action is just a UI interface for telling a extension what to do. ",
    },
    {
        id: 4,
        question: "Whats WASM and why does Anything use it?",
        answer:
            "Anything plugins are written in WASM. In simplest terms its great new tech  to run code from lots of different software langauges making it easier to write plugins for Anything. It also helps make it secure.",
    },
    {
        id: 5,
        question: "Whats languages can extensions be written in?",
        answer:
            "Currently Rust but we plan to support Javascript, Go, C#, F#, C, Haskell, Zig & AssemblyScript.",
    },
]

export default function Example() {
    return (
        <div className="">
            <div className="mx-auto max-w-7xl px-6 py-16 sm:py-24 lg:px-8">
                <h2 className="text-2xl font-bold leading-10 tracking-tight text-white">Frequently asked questions</h2>
                <p className="mt-6 max-w-2xl text-base leading-7 text-gray-300">
                    Have fun complicated questions? Reach out to us{' '}
                    <a href="https://discord.gg/VRBKaqjprE" className="font-semibold text-indigo-400 hover:text-indigo-300">
                        on Discord.
                    </a>
                </p>
                <div className="mt-20">
                    <dl className="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                        {faqs.map((faq) => (
                            <div key={faq.id}>
                                <dt className="text-base font-semibold leading-7 text-white">{faq.question}</dt>
                                <dd className="mt-2 text-base leading-7 text-gray-300">{faq.answer}</dd>
                            </div>
                        ))}
                    </dl>
                </div>
            </div>
        </div>
    )
}
