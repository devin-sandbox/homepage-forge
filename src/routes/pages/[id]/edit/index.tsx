import { component$ } from '@builder.io/qwik';
import { routeLoader$, Form, useLocation } from '@builder.io/qwik-city';

export const usePage = routeLoader$(async ({ params }) => {
  const response = await fetch(`http://localhost:8080/api/pages/${params.id}`);
  return await response.json();
});

export default component$(() => {
  const page = usePage();
  const loc = useLocation();

  return (
    <div class="p-4">
      <h1 class="text-2xl font-bold mb-4">Edit Page</h1>
      <Form
        onSubmit$={async (event) => {
          const formData = new FormData(event.target as HTMLFormElement);
          await fetch(`http://localhost:8080/api/pages/${loc.params.id}`, {
            method: 'PUT',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({
              title: formData.get('title'),
              content: formData.get('content'),
            }),
          });
          location.href = '/';
        }}
      >
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium mb-1">Title</label>
            <input
              type="text"
              name="title"
              required
              value={page.value.title}
              class="input"
            />
          </div>
          <div>
            <label class="block text-sm font-medium mb-1">Content</label>
            <textarea
              name="content"
              required
              rows={10}
              value={page.value.content}
              class="input"
            />
          </div>
          <div>
            <button
              type="submit"
              class="button"
            >
              Update Page
            </button>
          </div>
        </div>
      </Form>
    </div>
  );
});
