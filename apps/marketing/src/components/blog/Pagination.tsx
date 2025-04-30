interface PaginationProps {
  slug: string;
  pageNumber: number;
  lastPage: number;
}

const Pagination: React.FC<PaginationProps> = ({
  slug,
  pageNumber,
  lastPage,
}) => {
  return (
    <div className="flex mt-12 items-center justify-center text-sm">
      <a
        className={`border border-gray-200 dark:border-slate-700 rounded-md px-4 py-2 w-[90px] text-center bg-white dark:bg-slate-800 text-gray-800 dark:text-slate-200 hover:bg-pink-50 dark:hover:bg-pink-950 hover:text-pink-600 dark:hover:text-pink-400 hover:border-pink-200 dark:hover:border-pink-800 transition-all font-medium ${
          pageNumber ? "" : "pointer-events-none opacity-30"
        }`}
        href={pageNumber ? `${slug}?page=${pageNumber}` : "#"}
      >
        ← Prev
      </a>
      <div className="px-6 font-bold text-gray-900 dark:text-slate-200">
        {pageNumber + 1} / {lastPage}
      </div>
      <a
        className={`border border-gray-200 dark:border-slate-700 rounded-md px-4 py-2 w-[90px] text-center bg-white dark:bg-slate-800 text-gray-800 dark:text-slate-200 hover:bg-pink-50 dark:hover:bg-pink-950 hover:text-pink-600 dark:hover:text-pink-400 hover:border-pink-200 dark:hover:border-pink-800 transition-all font-medium ${
          pageNumber >= lastPage - 1 ? "pointer-events-none opacity-30" : ""
        }`}
        href={
          pageNumber >= lastPage - 1 ? "#" : `${slug}?page=${pageNumber + 2}`
        }
      >
        Next →
      </a>
    </div>
  );
};

export default Pagination;
