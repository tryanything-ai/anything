// import { Tag } from "../types/flow";

export const Tags = ({ tags }: { tags: any[] }) => {
  return (
    <div className="mb-2 flex gap-1">
      {tags.map((tag, index) => {
        return (
          <div className="badge badge-outline" key={index}>
            {tag.tag_label}
          </div>
        );
      })}
    </div>
  );
};
