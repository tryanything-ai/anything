import clsx from "clsx";
import { useLocation } from "react-router-dom";
import { IconType } from "react-icons";
import { Link } from "react-router-dom";

interface NavLinkProps {
  link: string;
  icon: IconType;
}

const NavLink: React.FC<NavLinkProps> = ({ link, icon: Icon }) => {
  const location = useLocation();
  const selectedButtonClass = ""; 
  const defaultButtonClass = "w-full h-full p-2";
  const linkClass = "w-full h-10 border-r border-transparent";
  const selectedLinkClass = "w-full h-10 border-r border-crimson-9";

  return (
    <Link
      to={link}
      className={location.pathname === link ? selectedLinkClass : linkClass}
    >
      <Icon
        className={clsx(defaultButtonClass, {
          [selectedButtonClass]: location.pathname === link,
        })}
      />
    </Link>
  );
};

export default NavLink;
