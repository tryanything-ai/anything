const faqs = [
    {
        id: 1,
        question: "If its open source how do you make money?",
        answer:
            "We charge to run the workflow engine and provide other useful services.",
    },
    {
        id: 2,
        question: "Whats a plugin?",
        answer:
            "A plugin represents one \"action\" in an Anything workflow.",
    },
    {
        id: 3,
        question: "Whats WASM and why does Anything use it?",
        answer:
            "Anything plugins are written in WASM. In simplest terms its great new tech  to run code from lots of different software langauges making it easier to write plugins for Anything. It also helps make it secure.",
    },
    {
        id: 4,
        question: "Whats languages can plugins be written in?",
        answer:
            "Currently Rust, Javascript, Go, C#, F#, C, Haskell, Zig & AssemblyScript.",
    },
]

export default function Faq() {
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
