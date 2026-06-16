# Gallery Implementation Diagram

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Pixelsort Application                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐    │
│  │  Input   │ → │   Edit   │ ⇄ │  Crop    │   │ Gallery  │ ←┐ │
│  │  Phase   │   │  Phase   │   │  Phase   │   │  Phase   │  │ │
│  └──────────┘   └─────┬────┘   └──────────┘   └────┬─────┘  │ │
│                       │                              │        │ │
│                       │ [Gallery Button]             │ [X]    │ │
│                       └──────────────────────────────┘        │ │
│                                                                │ │
├─────────────────────────────────────────────────────────────────┤
│                        Gallery Module                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────┐    │
│  │                   GalleryState                          │    │
│  ├────────────────────────────────────────────────────────┤    │
│  │  • images: Vec<GalleryImage>                           │    │
│  │  • selected_indices: Vec<usize>                        │    │
│  │  • texture_cache: HashMap<PathBuf, TextureHandle>      │    │
│  │  • needs_refresh: bool                                 │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
│  ┌────────────────────────────────────────────────────────┐    │
│  │              Gallery UI Components                      │    │
│  ├────────────────────────────────────────────────────────┤    │
│  │                                                         │    │
│  │  ┌──────────────────────────────────────────────────┐  │    │
│  │  │ Top Bar                                          │  │    │
│  │  │  • Title: "Image Gallery"                       │  │    │
│  │  │  • Count: "X images"                            │  │    │
│  │  │  • Close: [X]                                   │  │    │
│  │  └──────────────────────────────────────────────────┘  │    │
│  │                                                         │    │
│  │  ┌──────────────────────────────────────────────────┐  │    │
│  │  │ Grid View (Scrollable)                          │  │    │
│  │  │                                                  │  │    │
│  │  │  [Img1]  [Img2]  [Img3]  [Img4]  [Img5]        │  │    │
│  │  │                                                  │  │    │
│  │  │  [Img6]  [Img7]  [Img8]  [Img9]  [Img10]       │  │    │
│  │  │                                                  │  │    │
│  │  │  • 280x280px thumbnails                         │  │    │
│  │  │  • Responsive columns (3-6)                     │  │    │
│  │  │  • Texture caching                              │  │    │
│  │  └──────────────────────────────────────────────────┘  │    │
│  │                                                         │    │
│  │  ┌──────────────────────────────────────────────────┐  │    │
│  │  │ Action Bar                                       │  │    │
│  │  │                                                  │  │    │
│  │  │   ☑ Select   ☐ Clear   💾 Export   🗑 Delete   │  │    │
│  │  │           All                                    │  │    │
│  │  │                                                  │  │    │
│  │  │          "X selected" counter                   │  │    │
│  │  └──────────────────────────────────────────────────┘  │    │
│  │                                                         │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Data Flow

```
User Action                Gallery State           File System
─────────────────────────  ─────────────────────  ─────────────────

Open Gallery
    │
    ├──────────────────→   refresh_images()
    │                          │
    │                          ├─────────────────→ Read sorted_images/
    │                          │                    session_*/
    │                          │                    *.png files
    │                          │
    │                          ├──────────────────→ Load thumbnails
    │                          │                    (with caching)
    │                          ↓
    └──────────────────→   Display grid

Select Image
    │
    └──────────────────→   toggle_selection()
                              └→ Update selected_indices

Select All
    │
    └──────────────────→   select_all()
                              └→ selected_indices = [0..n]

Delete Selected
    │
    ├──────────────────→   delete_selected()
    │                          │
    │                          ├─────────────────→ Delete files
    │                          │                    Clean up empty dirs
    │                          │
    │                          ├──────────────────→ Clear texture cache
    │                          │
    │                          └→ refresh_images()
    │
    └──────────────────→   Show message
                           "✓ Deleted X images"

Export to USB
    │
    ├──────────────────→   export_selected_to_usb()
    │                          │
    │                          ├─────────────────→ Find USB mount
    │                          │                    /media/*/
    │                          │
    │                          ├─────────────────→ Create export dir
    │                          │                    pixelsort_export/
    │                          │                    session_*/
    │                          │
    │                          └─────────────────→ Copy selected files
    │
    └──────────────────→   Show message
                           "✓ Exported X images"

Close Gallery
    │
    └──────────────────→   current_phase = Edit
```

## File Structure

```
sorted_images/
├── session_20260130_100000/
│   ├── edit_001_horizontal.png  ◄─── Displayed in gallery
│   ├── edit_002_vertical.png    ◄─── With cached thumbnail
│   └── edit_003_diagonal.png    ◄─── Tap to select
│
├── session_20260130_110000/
│   ├── edit_001_horizontal.png
│   ├── edit_002_horizontal.png
│   └── edit_003_horizontal.png
│
└── session_20260130_120000/
    └── edit_001_horizontal.png

USB Export Structure:
/media/usb_drive/
└── pixelsort_export/
    ├── session_20260130_100000/  ◄─── Preserves structure
    │   ├── edit_001_horizontal.png
    │   └── edit_002_vertical.png
    │
    └── session_20260130_110000/
        └── edit_001_horizontal.png
```

## Code Structure

```
src/ui/
├── gallery.rs (NEW)           ← Gallery implementation
│   ├── GalleryState struct
│   ├── GalleryImage struct
│   ├── Selection functions
│   ├── Delete functions
│   ├── Export functions
│   └── Rendering functions
│
├── mod.rs (MODIFIED)          ← Added gallery module
│   └── gallery_state field
│
├── state.rs (MODIFIED)        ← Added Gallery phase
│   └── Phase::Gallery
│
├── layouts.rs (MODIFIED)      ← Added Gallery button
│   └── Gallery button in Edit phase
│
└── viewport.rs (MODIFIED)     ← Added Gallery rendering
    └── Phase::Gallery case
```

## Performance Optimizations

```
┌─────────────────────────────────────────────────┐
│            Texture Cache Strategy                │
├─────────────────────────────────────────────────┤
│                                                  │
│  First Render:                                  │
│  ┌──────────┐    ┌─────────┐    ┌──────────┐  │
│  │ Load PNG │ →  │ Resize  │ →  │  Cache   │  │
│  │   File   │    │  280px  │    │ Texture  │  │
│  └──────────┘    └─────────┘    └──────────┘  │
│       ↓                                ↑        │
│    Disk I/O                         GPU         │
│    ~50ms                                        │
│                                                  │
│  Subsequent Renders:                            │
│  ┌──────────┐                                   │
│  │  Reuse   │  ← From cache (< 1ms)           │
│  │ Texture  │                                   │
│  └──────────┘                                   │
│                                                  │
│  On Delete/Refresh:                             │
│  ┌──────────┐                                   │
│  │  Clear   │  ← Free GPU memory               │
│  │  Cache   │                                   │
│  └──────────┘                                   │
│                                                  │
└─────────────────────────────────────────────────┘
```

## Touch Interaction Flow

```
User Taps Image
       │
       ↓
   [Response]
       │
       ├─ Hovered? ──→ Show grey highlight
       │
       ├─ Clicked? ──→ toggle_selection()
       │                    │
       │                    ├─ Is Selected?
       │                    │     │
       │                    │     ├─ Yes → Deselect
       │                    │     │        └→ Remove from selected_indices
       │                    │     │
       │                    │     └─ No → Select
       │                    │              └→ Add to selected_indices
       │                    │
       │                    └─ Update UI
       │                         ├→ Blue background
       │                         └→ Green checkmark
       │
       └─ Released? ──→ Reset interaction state
```

## Success Flow

```
1. User creates pixel art
        ↓
2. Images auto-saved to sorted_images/
        ↓
3. User clicks Gallery button
        ↓
4. Gallery loads and displays thumbnails
        ↓
5. User selects images (tap to toggle)
        ↓
6. User performs action:
   ├─ Export to USB → Images copied
   └─ Delete → Images removed
        ↓
7. Gallery updates automatically
        ↓
8. User clicks X to return to Edit
```

---

**Implementation Complete** ✅
- 700+ lines of new code
- 24 lines modified in existing files
- 0 new dependencies
- Full documentation provided
- Ready for testing
