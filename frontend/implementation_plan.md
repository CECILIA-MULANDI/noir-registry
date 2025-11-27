# Noir Registry Frontend Redesign Plan

## Goal Description
Redesign the Noir Registry frontend to achieve a modern, premium, and dark-themed aesthetic. The current UI is functional but lacks visual polish. We will improve typography, spacing, colors, and component structure.

## User Review Required
> [!IMPORTANT]
> I will be refactoring the single `page.tsx` file into multiple components (`Header`, `Hero`, `PackageCard`, `Footer`) for better maintainability.

## Proposed Changes

### Design System
#### [MODIFY] [globals.css](file:///home/cecilia/Developer/Lynette_Cecilia_Projects/v2_noir_registry/frontend/src/app/globals.css)
- Update CSS variables for a refined dark palette (using rich grays/blacks).
- Add `Inter` font family.
- Add modern CSS reset.

### Components
#### [NEW] [Header.tsx](file:///home/cecilia/Developer/Lynette_Cecilia_Projects/v2_noir_registry/frontend/src/app/components/Header.tsx)
- Extract header from `page.tsx`.
- Improve styling (blur effect, better spacing).

#### [NEW] [Hero.tsx](file:///home/cecilia/Developer/Lynette_Cecilia_Projects/v2_noir_registry/frontend/src/app/components/Hero.tsx)
- Extract hero section.
- Improve search bar design and call-to-action buttons.

#### [NEW] [PackageCard.tsx](file:///home/cecilia/Developer/Lynette_Cecilia_Projects/v2_noir_registry/frontend/src/app/components/PackageCard.tsx)
- Create a reusable card component for package items.
- Add hover effects and better typography.

#### [NEW] [Footer.tsx](file:///home/cecilia/Developer/Lynette_Cecilia_Projects/v2_noir_registry/frontend/src/app/components/Footer.tsx)
- Extract footer.

### Pages
#### [MODIFY] [page.tsx](file:///home/cecilia/Developer/Lynette_Cecilia_Projects/v2_noir_registry/frontend/src/app/page.tsx)
- Reassemble the page using the new components.
- Update layout to be more responsive and spacious.

## Verification Plan
### Automated Tests
- Run `npm run build` to ensure type safety and build success.
- (Optional) If there are existing tests, run them.

### Manual Verification
- Since I cannot see the browser, I will rely on the user to verify the visual changes.
- I will ensure the code structure is clean and follows best practices.
