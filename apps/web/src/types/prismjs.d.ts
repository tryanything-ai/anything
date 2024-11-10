declare module 'prismjs/components/prism-core' {
    const Prism: {
        highlight: (code: string, grammar: any, language: string) => string;
        languages: any;
    };
    export const highlight: (code: string, grammar: any, language: string) => string;
    export const languages: any;
    export default Prism;
}