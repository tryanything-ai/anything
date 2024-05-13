// import {
//     ArrowPathIcon,
//     CloudArrowUpIcon,
//     Cog6ToothIcon,
//     FingerPrintIcon,
//     LockClosedIcon,
//     ServerIcon,
// } from '@heroicons/react/20/solid'

import { VscArrowUp, VscBell, VscDatabase, VscFolderOpened, VscGithub, VscGithubAlt, VscHeart, VscLock, VscTerminal } from "react-icons/vsc"

const features = [
    {
        name: 'Simple State Management.',
        description: 'Triggers, Actions, and Flow definitions are stored in files on your computer. Ready to be managed in Git. ',
        icon: VscFolderOpened,
    },
    {
        name: 'No Docker Required.',
        description: 'Docker is great for infrastructure but sucks for apps! Use Anything like a normal program. Just download and go.',
        icon: VscHeart,
    },
    {
        name: 'Local Sqlite Database.',
        description: 'Keep it simple. Nothing fancy here. Thank goodness.',
        icon: VscDatabase,
    },
    {
        name: 'File Change Triggers.',
        description: 'Watch local files and run workflows based on changes.',
        icon: VscBell,
    },
    {
        name: 'Run CLI Commands.',
        description: 'Anything you can do it can do better. Well it CAN do it is the point for now.',
        icon: VscTerminal,
    },
    {
        name: "Extremely Open Source.",
        description: "The code is public. The vibes are freedom. Consider this your new swiss army knife.",
        icon: VscGithubAlt
    },
    // {
    //     name: "Remix. Not Rebuild.",
    //     description: "Built to let you reuse more work. Its almost like programming where every function you ever written can be reuised in every future program you write.",
    //     icon: VscGithubAlt
    // },
    // {
    //     name: 'Simple queues.',
    //     description: 'Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus voluptas blanditiis et.',
    //     icon: ArrowPathIcon,
    // },
    // {
    //     name: 'Advanced security.',
    //     description: 'Iure sed ab. Aperiam optio placeat dolor facere. Officiis pariatur eveniet atque et dolor.',
    //     icon: FingerPrintIcon,
    // },
    // {
    //     name: 'Powerful API.',
    //     description: 'Laudantium tempora sint ut consectetur ratione. Ut illum ut rem numquam fuga delectus.',
    //     icon: Cog6ToothIcon,
    // },
    // {
    //     name: 'Database backups.',
    //     description: 'Culpa dolorem voluptatem velit autem rerum qui et corrupti. Quibusdam quo placeat.',
    //     icon: ServerIcon,
    // },
]

export default function Example() {
    return (

        <div className="grid py-24 sm:py-32">
            <div className="mx-auto max-w-6xl px-6 lg:px-8">
                <div className=" max-w-2xl lg:mx-0">
                    {/* <h2 className="text-base font-semibold leading-7 text-indigo-400">Everything you need</h2> */}
                    <p className="mt-2 text-3xl font-bold tracking-tight text-white sm:text-4xl">Not everything is better on a  <span className="text-crimson-9">server</span>.</p>
                    <p className="mt-6 text-lg leading-8 text-gray-300">
                        Sick of SaaS bills? Censored AI Models? We are too. That's why we built a tool that lets you run your own automations. No server required.
                    </p>
                </div>
                <dl className="mx-auto mt-16 grid max-w-xl grid-cols-1 gap-8 text-base leading-7 text-gray-300 lg:grid-cols-2 lg:mx-0 lg:max-w-none lg:gap-x-16">
                    {features.map((feature) => (
                        <div key={feature.name} className="relative pl-9">
                            <dt className="inline font-semibold text-white">
                                <feature.icon className="absolute left-1 top-1 h-5 w-5 text-indigo-500" aria-hidden="true" />
                                {feature.name}
                            </dt>{' '}
                            <dd className="inline">{feature.description}</dd>
                        </div>
                    ))}
                </dl>
            </div>
        </div>
    )
}
