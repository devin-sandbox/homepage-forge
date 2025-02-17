import { component$ } from '@builder.io/qwik';
import { Form } from '@builder.io/qwik-city';

export default component$(() => {
  return (
    <div class="p-4">
      <h1 class="text-2xl font-bold mb-4">Create New Page</h1>
      <Form
        onSubmit$={async (event) => {
          const formData = new FormData(event.target as HTMLFormElement);
          await fetch('http://localhost:8080/api/pages', {
            method: 'POST',
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
              class="input"
            />
          </div>
          <div>
            <label class="block text-sm font-medium mb-1">Content</label>
            <textarea
              name="content"
              required
              rows={10}
              class="input"
            />
          </div>
          <div>
            <button
              type="submit"
              class="button"
            >
              Create Page
            </button>
          </div>
        </div>
      </Form>
    </div>
  );
});
