import { newSpecPage } from '@stencil/core/testing';
import { TaflBrandubh } from '../tafl-brandubh';

describe('tafl-brandubh', () => {
  it('renders', async () => {
    const page = await newSpecPage({
      components: [TaflBrandubh],
      html: `<tafl-brandubh></tafl-brandubh>`,
    });
    expect(page.root).toEqualHtml(`
      <tafl-brandubh>
        <mock:shadow-root>
          <slot></slot>
        </mock:shadow-root>
      </tafl-brandubh>
    `);
  });
});
