-- migrate:up
ALTER TABLE packages
DROP CONSTRAINT packages_inventory_item_id_fkey,
ADD CONSTRAINT packages_inventory_item_id_fkey
    FOREIGN KEY (inventory_item_id) REFERENCES inventory(id) ON DELETE CASCADE;

-- migrate:down
ALTER TABLE packages
DROP CONSTRAINT packages_inventory_item_id_fkey,
ADD CONSTRAINT packages_inventory_item_id_fkey
    FOREIGN KEY (inventory_item_id) REFERENCES inventory(id);

