Feature: Wallpaper Loading
  As a user of ThingOS
  I want the desktop wallpaper to load and display correctly
  So that I have a pleasant visual experience

  Background:
    Given the bloom compositor is initialized
    And a valid BMP wallpaper bytespace exists

  Scenario: Successfully load and parse a BMP wallpaper
    Given a valid 24-bit BMP file with dimensions 1024x768
    When the wallpaper loader parses the BMP file
    Then the parser should return pixel data with 786432 pixels
    And each pixel should have an alpha channel value of 255

  Scenario: Handle invalid BMP magic bytes
    Given a file with invalid BMP magic bytes
    When the wallpaper loader attempts to parse the file
    Then the parser should return an error "Invalid BMP magic"

  Scenario: Handle undersized BMP file
    Given a file smaller than 54 bytes
    When the wallpaper loader attempts to parse the file
    Then the parser should return an error "BMP too small"

  Scenario: Handle unsupported bit depth
    Given a BMP file with 16-bit color depth
    When the wallpaper loader attempts to parse the file
    Then the parser should return an error "Unsupported BMP bit depth"

  Scenario: Use actual mapped address instead of requested address
    Given the bytespace mapping system is available
    When space_map is called with requested address 0x85000000
    And space_map returns actual address 0x86000000
    Then the loader should use address 0x86000000 for data access
    And the loader should not use address 0x85000000

  Scenario: Detect space_map failure
    Given the bytespace mapping system is available
    When space_map returns 0
    Then the loader should report a mapping failure
    And the wallpaper should not be displayed
