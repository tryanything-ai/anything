export function TemplateCard({ template }: { template: any }) {
  return (
    <a
      href={template.link}
      className="bg-base-300 rounded-lg overflow-hidden transition-all duration-200 ease-in-out transform hover:scale-105"
    >
      {/* <img
        src={template.image}
        alt={template.name}
        className="w-full h-48 object-cover"
      /> */}
      <div className="p-6">
        <h2 className="text-lg font-semibold text-gray-700">{template.name}</h2>
        <p className="text-gray-500">{template.description}</p>
      </div>
    </a>
  );
}
