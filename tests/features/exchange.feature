Feature: Get information from Exchange API
  As a third party developer
  I want to get relevant information from the Exchange API
  So that my users can make better decisions on their investments

  Scenario: The Exchange server time is correct
    When I request the server time
    Then I get a proper server time

  Scenario: I can get a pair trading info
    When I request the "XBTUSD" trading pair
    Then I get proper trading "XBTUSD" info

  Scenario: I can get a pair ticker
    When I request the "XBTUSD" ticker
    Then I get proper ticker "XBTUSD" info

    # Since I don't currently have open orders, I request the list,
    # validate the response format and check that the list is an empty object
  Scenario: I can get my open orders
    Given I have a 2FA account
    When I request the open orders
    Then I get my list of open orders
