# Gallery Feature Documentation

## Overview
The Gallery feature provides an image organization system for viewing, managing, and exporting pixelsorted images. It's accessible from the Edit phase via a "Gallery" button.

## Features Implemented

### 1. Gallery Phase
- New `Phase::Gallery` added to the phase system
- Accessible via "Gallery" button in Edit phase (bottom row, rightmost position)
- Full-screen gallery view with top/bottom bars

### 2. Image Viewing
- **Grid Layout**: Images displayed in a responsive grid (3-6 columns depending on screen width)
- **Thumbnails**: 280x280px thumbnails with automatic scaling
- **Session Organization**: Images grouped by session folder
- **Filename Display**: Shows filename below each thumbnail (truncated if too long)

### 3. Selection System
- **Touch-friendly Selection**: Tap any image to toggle selection
- **Visual Feedback**: 
  - Selected images have blue background
  - Checkmark icon in top-right corner
  - Hover effect on unselected images
- **Batch Selection**: 
  - "Select All" button (☑) - selects all images
  - "Clear Selection" button (☐) - deselects all images
- **Selection Counter**: Shows "X selected" at bottom when items are selected

### 4. Actions
Five action buttons in the bottom bar:

#### Select All (☑)
- Color: Blue (#4678B4)
- Selects all images in the gallery

#### Clear Selection (☐)
- Color: Grey (disabled when no selection)
- Deselects all images

#### Export to USB (💾)
- Color: Green (disabled when no selection)
- Exports selected images to USB drive
- Only works on Linux with mounted USB
- Shows success/failure message

#### Delete (🗑)
- Color: Red (disabled when no selection)
- Deletes selected images from disk
- Cleans up empty session folders
- Shows confirmation with count

#### Refresh (🔄)
- Color: Grey
- Refreshes the gallery to show new images
- Useful after creating new images

### 5. Top Bar
- **Title**: "Image Gallery"
- **Image Count**: Shows total number of images
- **Close Button**: X button in top-right to return to Edit phase

## UI Design

### Layout
```
┌─────────────────────────────────────────────────┐
│  Image Gallery    8 images                   X  │  Top Bar (100px)
├─────────────────────────────────────────────────┤
│                                                 │
│  [Img1]  [Img2]  [Img3]  [Img4]                │
│                                                 │  Scrollable
│  [Img5]  [Img6]  [Img7]  [Img8]                │  Grid Area
│                                                 │
│                                                 │
├─────────────────────────────────────────────────┤
│       ☑    ☐    💾    🗑    🔄                 │  Bottom Bar (120px)
│              X selected                         │
└─────────────────────────────────────────────────┘
```

### Color Scheme
- **Background**: Dark grey (#141414)
- **Top/Bottom Bars**: Semi-transparent dark (#1E1E1E with 230 alpha)
- **Thumbnails**: 
  - Default: Dark grey (#282828)
  - Hover: Lighter grey (#3C3C3C)
  - Selected: Blue (#4678B4)
- **Text**: White/light grey

### Touch Optimization
- **Button Size**: 90px diameter (45px radius)
- **Spacing**: 30px between buttons
- **Thumbnail Size**: 280x280px (easy to tap)
- **Thumbnail Spacing**: 40px (prevents accidental taps)

## File Structure

### New Files
- `src/ui/gallery.rs` - Main gallery implementation (600+ lines)
  - `GalleryState` struct for state management
  - `GalleryImage` struct for image metadata
  - Grid rendering logic
  - Selection/deletion/export functionality

### Modified Files
- `src/ui/state.rs` - Added `Phase::Gallery`
- `src/ui/mod.rs` - Added gallery module and `gallery_state` field
- `src/ui/layouts.rs` - Added Gallery button, handled Gallery phase
- `src/ui/viewport.rs` - Added Gallery viewport rendering

## Implementation Details

### State Management
```rust
pub struct GalleryState {
    pub images: Vec<GalleryImage>,
    pub selected_indices: Vec<usize>,
    pub scroll_offset: f32,
    pub needs_refresh: bool,
}
```

### Image Discovery
1. Scans `sorted_images/` directory
2. Iterates through session folders
3. Finds all `.png` files
4. Sorts by path (newest sessions first)
5. Loads thumbnails on-demand

### Export to USB
- Linux only (checks `/media/` mount points)
- Supports exfat, vfat, ntfs filesystems
- Creates `pixelsort_export/` folder on USB
- Copies selected images with original filenames
- Shows success/failure messages

### Deletion
- Deletes selected files from disk
- Cleans up empty session folders automatically
- Updates gallery view after deletion
- Thread-safe file operations

## Usage Flow

1. **Enter Gallery**
   - From Edit phase, tap "Gallery" button
   - Gallery loads and displays all saved images

2. **Select Images**
   - Tap individual images to select/deselect
   - Or use "Select All" to select everything
   - Selected images show blue background + checkmark

3. **Perform Action**
   - **Export**: Tap 💾 to copy to USB (USB must be mounted)
   - **Delete**: Tap 🗑 to remove images (permanent!)
   - **Clear**: Tap ☐ to deselect all

4. **Exit Gallery**
   - Tap X in top-right corner
   - Returns to Edit phase

## Testing

### Test Images Created
Located in `sorted_images/` (git-ignored):
- `session_20260130_100000/` - 3 images
- `session_20260130_110000/` - 5 images

### Manual Testing Steps
1. Run the application
2. Navigate to Edit phase (take/upload an image first)
3. Click "Gallery" button
4. Verify:
   - [ ] Images load in grid
   - [ ] Selection works (tap to select/deselect)
   - [ ] Select All works
   - [ ] Clear Selection works
   - [ ] Refresh updates the view
   - [ ] Close button returns to Edit phase
   - [ ] (On Linux with USB) Export works
   - [ ] Delete removes images and shows count

## Future Enhancements (Optional)
- Image preview on long-press
- Sort options (by date, name, algorithm)
- Search/filter functionality
- Multi-touch gestures (pinch to zoom)
- Image metadata display
- Undo deletion
- Session management (rename, merge sessions)

## Technical Notes

### Performance
- Thumbnails generated on-the-fly using `image::thumbnail()`
- Texture caching via egui's texture system
- Lazy loading (only visible images are rendered)
- Scrolling with `egui::ScrollArea`

### Platform Support
- **Desktop**: Fully functional
- **Raspberry Pi**: Touch-optimized for 1920x1080 touchscreen
- **USB Export**: Linux only (Pi target)

### Dependencies
No new dependencies added - uses existing:
- `eframe/egui` for UI
- `image` for thumbnail generation
- Standard library for file operations
