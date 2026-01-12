# Blossom (Display Broker)

Blossom is the stable contract for Bloom. It never maps pixel memory and never touches hardware.

## Responsibilities

- Accept driver registration over the Driver Protocol v0.
- Select the active driver (v0: first registered).
- Serve Bloom over the Display Protocol v0.
- Forward PRESENT messages to the active driver.

## Guarantees

- No bytespace mapping.
- No device claims or MMIO.
- Only forwards messages and metadata.

## Inputs

- Display protocol ports from Sprout:
  - DISPLAY_REQ_PORT (Bloom -> Blossom)
  - DISPLAY_RESP_PORT (Blossom -> Bloom)
- Driver protocol ports from Sprout:
  - DRV_REQ_PORT (Blossom -> Driver)
  - DRV_RESP_PORT (Driver -> Blossom)
- Compositor bytespace discovered via graph properties.

## Output

- INFO/BUFFER metadata responses to Bloom.
- PRESENT forwarded to the driver.
