import { component$ } from '@builder.io/qwik';
import { routeLoader$, type DocumentHead } from '@builder.io/qwik-city';

export const usePages = routeLoader$(async () => {
  const response = await fetch('http://localhost:8080/api/pages');
  const pages = await response.json();
  return pages;
});

export default component$(() => {
  const pages = usePages();

  return (
    <div class="p-4">
      <h1 class="text-2xl font-bold mb-4">Pages</h1>
      <div style="margin-bottom: 1rem;">
        <a href="/pages/new" class="button">
          Create New Page
        </a>
      </div>
      <div class="space-y-4">
        {pages.value.map((page: any) => (
          <div key={page.id} class="card">
            <h2>{page.title}</h2>
            <div>
              <a href={`/pages/${page.id}/edit`} class="button">
                Edit
              </a>
              <button
                onClick$={async () => {
                  if (confirm('Are you sure you want to delete this page?')) {
                    await fetch(`http://localhost:8080/api/pages/${page.id}`, {
                      method: 'DELETE',
                    });
                    location.reload();
                  }
                }}
                class="button button-delete"
              >
                Delete
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
});

export const head: DocumentHead = {
  title: 'Homepage Creator',
};
