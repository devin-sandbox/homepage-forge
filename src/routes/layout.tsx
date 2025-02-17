import { component$, Slot } from '@builder.io/qwik';

export default component$(() => {
  return (
    <div>
      <header class="header">
        <div class="container">
          <h1>
            <a href="/">Homepage Creator</a>
          </h1>
        </div>
      </header>
      <main class="container">
        <Slot />
      </main>
    </div>
  );
});
