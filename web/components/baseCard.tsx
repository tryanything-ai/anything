import React, { FC, HTMLAttributes, ComponentType, ReactNode } from "react";
import clsx from "clsx";

interface LinkProps {
  href: string;
}

type ConditionalProps<T> = T extends { to: string } ? LinkProps : {};

interface BaseCardProps<T = HTMLElement> extends HTMLAttributes<T> {
  as?: React.ElementType;
  image?: ReactNode,
  className?: string;
}

const BaseCard: FC<BaseCardProps & ConditionalProps<any>> = ({
  as: Component = "div",
  children,
  image,
  className,
  ...props
}) => {
  const baseStyles = "card bg-base-300 shadow-xl my-2";

  const combinedStyles = clsx(baseStyles, className);

  return (
    <Component className={combinedStyles} {...props}>
      {image}
      <div className="card-body">{children}</div>
    </Component>
  );
};

export default BaseCard;
