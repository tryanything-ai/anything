
import { VscBell, VscDatabase, VscFolderOpened, VscGithubAlt, VscHeart, VscLock, VscRepoForked, VscSymbolColor, VscTerminal } from "react-icons/vsc"

const features = [
    {
        name: 'Automation Studio',
        description: 'Let your users build workflows with a visual editor. No code required.',
        icon: VscRepoForked,
    },
    {
        name: 'Template Marketplace',
        description: 'Create templates that align with your industry. Share them with your users and let them share too.',
        icon: VscHeart,
    },
    {
        name: 'Workflow Engine',
        description: 'Run workflows on our infrastructure. No need to worry about scaling.',
        icon: VscTerminal,
    },
    {
        name: 'Secrets Management',
        description: 'Store and manage secrets for your users. Keep their data safe.',
        icon: VscLock,
    },
    {
        name: 'Infinite Customization',
        description: 'Build custom plugins in 7 languages. Our servers can run them. The sky is the limit.',
        icon: VscSymbolColor,
    },
    {
        name: "Open Source",
        description: "If we ever screw up or become evil you can always self host. No platform risk. No lock in.",
        icon: VscGithubAlt
    }
]

export default function Features() {
    return (

        <div className="grid py-24 sm:py-32">
            <div className="mx-auto max-w-6xl px-6 lg:px-8">
                <div className=" max-w-2xl lg:mx-0">
                    {/* <h2 className="text-base font-semibold leading-7 text-indigo-400">Everything you need</h2> */}
                    <p className="mt-2 text-3xl font-bold tracking-tight text-white sm:text-4xl">Like <span className="text-crimson-9">Shopify</span> but for building a niche AI Automation Platform</p>
                    <p className="mt-6 text-lg leading-8 text-gray-300">
                        Shared infrastructure. Shared upside. Minimal engineering effort.
                    </p>
                </div>
                <dl className="mx-auto mt-16 grid max-w-xl grid-cols-1 gap-8 text-base leading-7 text-gray-300 lg:grid-cols-2 lg:mx-0 lg:max-w-none lg:gap-x-16">
                    {features.map((feature) => (
                        <div key={feature.name} className="relative pl-9">
                            <dt className="inline font-semibold text-white">
                                <feature.icon className="absolute left-1 top-1 h-5 w-5 text-indigo-500" aria-hidden="true" />
                                {feature.name}
                            </dt>{' '}
                            <br />
                            <dd className="inline">{feature.description}</dd>
                        </div>
                    ))}
                </dl>
            </div>
        </div>
    )
}
