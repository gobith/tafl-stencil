import { newE2EPage } from '@stencil/core/testing';

describe('tafl-brandubh', () => {
  it('renders', async () => {
    const page = await newE2EPage();
    await page.setContent('<tafl-brandubh></tafl-brandubh>');

    const element = await page.find('tafl-brandubh');
    expect(element).toHaveClass('hydrated');
  });
});
