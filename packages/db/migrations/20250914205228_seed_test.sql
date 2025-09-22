-- Insert a test user
INSERT INTO users (id, username, email)
VALUES (
    gen_random_uuid(),
    'alice',
    'alice@tidasone.com'
);

-- Insert inventory item
INSERT INTO inventory (id, owner_id, name, description, quantity, location, token_id)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM users LIMIT 1),
    'Space Drone',
    'Autonomous security drone',
    5,
    'Mars Outpost 1',
    'token123'
);

-- Insert package
INSERT INTO packages (id, owner_id, inventory_item_id, status, destination, nft_token)
VALUES (
    gen_random_uuid(),
    (SELECT id FROM users LIMIT 1),
    (SELECT id FROM inventory LIMIT 1),
    'Pending',
    'Lunar Base Alpha',
    'nft-001'
);

