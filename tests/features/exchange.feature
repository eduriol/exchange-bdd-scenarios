Feature: Get information from Exchange API
  As a third party developer
  I want to get relevant information from the Exchange API
  So that my users can make better decisions on their investments

  Scenario: The Exchange server time is correct
    When I request the server time
    Then I get a proper server time

  Scenario: I can get the bitcoin and dollar trading info
    When I request the XBT/USD trading pair
    Then I get proper trading pair info
