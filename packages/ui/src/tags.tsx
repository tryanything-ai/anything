import { Tag } from "utils";

export const Tags = ({ tags }: { tags: Tag[] }) => {
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
