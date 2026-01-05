Feature: RTC Driver

  Scenario: RTC Driver publishes device node
    Given the machine is booted
    Then a Thing named "device.rtc0" should exist
    And the Thing "device.rtc0" should be in "place.devices"
    And the Thing "device.rtc0" should have kind "kind.RtcDevice"
    # And the payload of "device.rtc0" should match schema "RtcDevice"
