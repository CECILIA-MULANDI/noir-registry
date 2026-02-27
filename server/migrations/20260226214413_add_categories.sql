-- Categories table
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT
);

-- Many-to-one: each package belongs to at most one category
CREATE TABLE package_categories (
    package_id INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (package_id, category_id)
);

CREATE INDEX idx_package_categories_package ON package_categories(package_id);
CREATE INDEX idx_package_categories_category ON package_categories(category_id);

-- Seed starting categories
INSERT INTO categories (name, slug, description) VALUES
    ('Cryptography', 'cryptography', 'Hashing, encryption, signatures, and crypto primitives'),
    ('Data Structures', 'data-structures', 'Trees, arrays, sets, and other data structures'),
    ('Math', 'math', 'Mathematical operations, number theory, and field arithmetic'),
    ('Utilities', 'utilities', 'General-purpose helper libraries and tools'),
    ('Zero Knowledge', 'zero-knowledge', 'ZK proof helpers, verifiers, and proof-system utilities'),
    ('Circuits', 'circuits', 'Reusable circuit components and gadgets'),
    ('Standards', 'standards', 'Implementations of standards (EIP, BIP, RFC, etc.)');
