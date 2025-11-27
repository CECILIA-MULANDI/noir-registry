import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Noir Registry - Package Registry for Noir",
  description: "The centralized package registry for the Noir programming language ecosystem",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}