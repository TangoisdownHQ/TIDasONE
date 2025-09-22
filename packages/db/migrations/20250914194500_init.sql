-- Users
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    nft_token_id TEXT,
    identity_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Inventory
CREATE TABLE inventory (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    description TEXT,
    quantity INT NOT NULL,
    location TEXT,
    token_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Packages
CREATE TABLE packages (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES users(id),
    inventory_item_id UUID REFERENCES inventory(id),
    status TEXT NOT NULL DEFAULT 'Pending',
    destination TEXT NOT NULL,
    nft_token TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

