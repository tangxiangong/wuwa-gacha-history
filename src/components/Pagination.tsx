import { For, Show } from "solid-js";

interface PaginationProps {
  currentPage: number;
  totalRecords: number;
  pageSize: number;
  onPageChange: (page: number) => void;
}

export default function Pagination(props: PaginationProps) {
  const totalPages = () => Math.max(1, Math.ceil(props.totalRecords / props.pageSize));

  const pageNumbers = () => {
    const total = totalPages();
    const current = props.currentPage;
    const pages: number[] = [];

    let start = Math.max(1, current - 2);
    let end = Math.min(total, start + 4);
    start = Math.max(1, end - 4);

    for (let i = start; i <= end; i++) {
      pages.push(i);
    }
    return pages;
  };

  return (
    <Show when={totalPages() > 1}>
      <div class="pagination">
        <button
          class="page-btn"
          disabled={props.currentPage <= 1}
          onClick={() => props.onPageChange(props.currentPage - 1)}
        >
          ‹
        </button>
        <For each={pageNumbers()}>
          {(page) => (
            <button
              class={`page-btn ${page === props.currentPage ? "active" : ""}`}
              onClick={() => props.onPageChange(page)}
            >
              {page}
            </button>
          )}
        </For>
        <button
          class="page-btn"
          disabled={props.currentPage >= totalPages()}
          onClick={() => props.onPageChange(props.currentPage + 1)}
        >
          ›
        </button>
      </div>
    </Show>
  );
}
