# Gallery Feature Testing Guide

## Build Instructions

### Prerequisites
On Raspberry Pi or Linux desktop:
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y libgtk-3-dev build-essential pkg-config
```

### Build
```bash
cd /home/runner/work/Pixelsort/Pixelsort
cargo build --release
```

### Run
```bash
cargo run --release
```

## Testing Checklist

### 1. Gallery Access
- [ ] Start application (or upload an image to reach Edit phase)
- [ ] Verify "Gallery" button appears in Edit phase (bottom row, far right)
- [ ] Click "Gallery" button
- [ ] Verify gallery screen loads

### 2. Image Display
- [ ] Verify images are displayed in a grid
- [ ] Check that thumbnails are properly sized (280x280px)
- [ ] Verify filenames are shown below each thumbnail
- [ ] Check that long filenames are truncated with "..."
- [ ] Verify images from different sessions are all shown

### 3. Top Bar
- [ ] Verify "Image Gallery" title is visible
- [ ] Check that image count is correct (e.g., "8 images")
- [ ] Verify X button is visible in top-right
- [ ] Click X button
- [ ] Confirm it returns to Edit phase

### 4. Selection Functionality

#### Individual Selection
- [ ] Click on an unselected image
- [ ] Verify it becomes selected (blue background + checkmark)
- [ ] Click the same image again
- [ ] Verify it becomes deselected (grey background, no checkmark)

#### Select All
- [ ] Click "Select All" button (☑)
- [ ] Verify all images are selected
- [ ] Check that selection counter shows correct count

#### Clear Selection
- [ ] With some images selected, click "Clear Selection" button (☐)
- [ ] Verify all images are deselected
- [ ] Check that button is greyed out when nothing is selected

### 5. Action Buttons

#### Refresh
- [ ] Click "Refresh" button (🔄)
- [ ] Verify gallery updates (useful after creating new images)

#### Export (Linux + USB only)
- [ ] Insert USB drive
- [ ] Select some images
- [ ] Click "Export" button (💾)
- [ ] Verify success message appears
- [ ] Check USB drive for `pixelsort_export/` folder
- [ ] Verify selected images are copied to USB
- [ ] Try without USB inserted
- [ ] Verify error message appears

#### Delete
- [ ] Select one or more images
- [ ] Click "Delete" button (🗑)
- [ ] Verify success message with count (e.g., "✓ Deleted 3 images")
- [ ] Verify images are removed from gallery
- [ ] Check filesystem to confirm deletion
- [ ] Verify empty session folders are cleaned up

### 6. Selection Counter
- [ ] Select 0 images - verify no counter shown
- [ ] Select 1 image - verify "1 selected" shown
- [ ] Select multiple images - verify "X selected" shown

### 7. Scrolling
- [ ] If more than 2 rows of images, scroll down
- [ ] Verify smooth scrolling
- [ ] Verify all images are accessible
- [ ] Scroll back up

### 8. Touch Interaction (Raspberry Pi)
- [ ] Verify all buttons respond to touch
- [ ] Check that button tap targets are large enough
- [ ] Verify selection works with touch
- [ ] Test scrolling with touch gestures

### 9. Edge Cases

#### Empty Gallery
- [ ] Delete all images
- [ ] Verify "No images found" message appears
- [ ] Verify "Create some pixel-sorted images to see them here" hint appears

#### Single Image
- [ ] Have only one image
- [ ] Verify it displays correctly
- [ ] Test selection/deletion with single image

#### Many Images (20+)
- [ ] Create many test images
- [ ] Verify grid layout handles multiple rows
- [ ] Test scrolling performance
- [ ] Verify memory usage is reasonable

## Creating Test Images

### Using Python Script
```bash
cd /home/runner/work/Pixelsort/Pixelsort
python3 << 'EOF'
from PIL import Image, ImageDraw
import os

# Create test session
session_dir = 'sorted_images/session_test'
os.makedirs(session_dir, exist_ok=True)

for i in range(1, 11):
    img = Image.new('RGB', (800, 600), (i*25, 100, 200-i*10))
    draw = ImageDraw.Draw(img)
    draw.text((350, 280), f"Test {i}", fill=(255, 255, 255))
    img.save(os.path.join(session_dir, f'edit_{i:03d}_horizontal.png'))
    print(f'Created test image {i}')
EOF
```

### Using the App
1. Take or upload an image
2. Apply pixel sorting
3. Click "Iterate" button (saves automatically)
4. Repeat to create multiple images
5. Open Gallery to see results

## Performance Testing

### Load Time
- [ ] Measure time to open gallery with 10 images
- [ ] Measure time to open gallery with 50 images
- [ ] Verify acceptable performance (< 2 seconds for 50 images)

### Memory Usage
- [ ] Monitor memory usage with many images
- [ ] Verify no memory leaks after repeated open/close
- [ ] Check thumbnail caching works correctly

### UI Responsiveness
- [ ] Verify UI remains responsive during scrolling
- [ ] Check that button presses are immediate
- [ ] Test rapid selection/deselection

## Integration Testing

### Session Workflow
- [ ] Create new photo session
- [ ] Create several iterations
- [ ] Open Gallery
- [ ] Verify all iterations appear
- [ ] Select and export some
- [ ] Return to Edit
- [ ] Create more iterations
- [ ] Refresh Gallery
- [ ] Verify new images appear

### USB Export Workflow
- [ ] Create images
- [ ] Insert USB
- [ ] Export all images
- [ ] Check USB contents
- [ ] Delete with "delete after export" option
- [ ] Verify local copies deleted but USB copy remains

## Known Limitations

1. **USB Export**: Linux only (uses `/media/` mount points)
2. **Thumbnail Generation**: On-demand (slight delay on first view)
3. **Large Images**: Very large source images may take time to thumbnail
4. **Touch Gestures**: Single-tap only (no pinch-to-zoom, long-press, etc.)

## Troubleshooting

### Gallery Won't Open
- Verify Phase::Gallery is added to state.rs
- Check that Gallery button click handler sets current_phase correctly
- Look for errors in console/logs

### Images Don't Appear
- Check that `sorted_images/` directory exists
- Verify images are .png format
- Check file permissions
- Try clicking Refresh button

### Selection Doesn't Work
- Verify gallery_state is initialized in PixelSorterApp::new()
- Check that index bounds are correct
- Look for borrow checker issues in logs

### Export Fails
- Verify USB is mounted under `/media/`
- Check filesystem type (exfat/vfat/ntfs supported)
- Verify write permissions on USB
- Check available space on USB

### Deletion Fails
- Check file permissions
- Verify images exist on disk
- Look for filesystem errors in logs
- Check if files are locked/in use

## Success Criteria

The gallery feature is complete when:
- ✓ Gallery is accessible from Edit phase
- ✓ All saved images are displayed in grid
- ✓ Selection system works (individual + select all/clear)
- ✓ Export to USB works on Linux
- ✓ Deletion works and cleans up empty folders
- ✓ UI is touch-friendly (large targets, good spacing)
- ✓ Performance is acceptable (< 2s load for 50 images)
- ✓ Returns to Edit phase correctly
- ✓ Messages show success/failure for actions
- ✓ No crashes or memory leaks
