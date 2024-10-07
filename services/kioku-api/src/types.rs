//! Shorthands for prisma's models

/// Prisma User Model
pub type User = prisma::user;
/// Prisma User Data
pub type UserData = User::Data;

/// Prisma Card Model
pub type Card = prisma::card;
/// Prisma Card Data
pub type CardData = Card::Data;

// Prisma Deck Model
pub type Deck = prisma::deck;
/// Prisma Deck Data
pub type DeckData = Deck::Data;
