describe('full e2e test', () => {
  it('has required ui elements', () => {
    cy.visit('/');

    cy.get('[data-cy="cypress-username"]')
      .should('exist')
      .should('have.value', 'username');

    cy.get('[data-cy="cypress-message"]')
      .should('exist')
      .should('have.value', '');

    cy.get('[data-cy="cypress-send"]')
      .should('exist');

    cy.get('[data-cy="cypress-refresh"]')
      .should('exist');

    cy.get('[data-cy="cypress-messages"]')
      .should('exist');
  });

  it('can send a message', () => {
    cy.visit('/');

    cy.get('[data-cy="cypress-username"]')
      .clear().type('Cypress');

    cy.get('[data-cy="cypress-message"]')
      .type('Hello there! i am cypress.');

    cy.get('[data-cy="cypress-send"]')
      .click();

    cy.wait(500);
  });
});
