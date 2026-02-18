/**
 * Generate app icons from Seshlog SVG logo
 * Run with: node scripts/generate-icons.mjs
 */

import sharp from 'sharp';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const SVG_SOURCE = path.join(__dirname, '../../docs/seshlog_icon.svg');
const ICONS_DIR = path.join(__dirname, '../src-tauri/icons');
const PUBLIC_DIR = path.join(__dirname, '../public');

// Icon sizes needed for Tauri
const ICON_SIZES = [
  { name: '32x32.png', size: 32 },
  { name: '128x128.png', size: 128 },
  { name: '128x128@2x.png', size: 256 },
  { name: 'icon.png', size: 512 },
  // Additional sizes for Windows Store
  { name: 'Square30x30Logo.png', size: 30 },
  { name: 'Square44x44Logo.png', size: 44 },
  { name: 'Square71x71Logo.png', size: 71 },
  { name: 'Square89x89Logo.png', size: 89 },
  { name: 'Square107x107Logo.png', size: 107 },
  { name: 'Square142x142Logo.png', size: 142 },
  { name: 'Square150x150Logo.png', size: 150 },
  { name: 'Square284x284Logo.png', size: 284 },
  { name: 'Square310x310Logo.png', size: 310 },
  { name: 'StoreLogo.png', size: 50 },
  // Additional icon sizes for various uses
  { name: 'icon_16x16.png', size: 16 },
  { name: 'icon_16x16@2x.png', size: 32 },
  { name: 'icon_32x32.png', size: 32 },
  { name: 'icon_32x32@2x.png', size: 64 },
  { name: 'icon_128x128.png', size: 128 },
  { name: 'icon_128x128@2x.png', size: 256 },
  { name: 'icon_256x256.png', size: 256 },
  { name: 'icon_256x256@2x.png', size: 512 },
  { name: 'icon_512x512.png', size: 512 },
  { name: 'icon_512x512@2x.png', size: 1024 },
];

async function generatePngIcons() {
  console.log('Reading SVG from:', SVG_SOURCE);

  if (!fs.existsSync(SVG_SOURCE)) {
    throw new Error(`SVG source not found: ${SVG_SOURCE}`);
  }

  // Ensure directories exist
  if (!fs.existsSync(ICONS_DIR)) {
    fs.mkdirSync(ICONS_DIR, { recursive: true });
  }
  if (!fs.existsSync(PUBLIC_DIR)) {
    fs.mkdirSync(PUBLIC_DIR, { recursive: true });
  }

  // Read SVG
  const svgBuffer = fs.readFileSync(SVG_SOURCE);

  console.log('\nGenerating PNG icons...');

  for (const { name, size } of ICON_SIZES) {
    const outputPath = path.join(ICONS_DIR, name);
    await sharp(svgBuffer)
      .resize(size, size, {
        fit: 'contain',
        background: { r: 255, g: 255, b: 255, alpha: 1 }
      })
      .png()
      .toFile(outputPath);
    console.log(`  ✓ ${name} (${size}x${size})`);
  }

  // Generate favicon for public folder
  const faviconPath = path.join(PUBLIC_DIR, 'favicon.ico');
  // For favicon, we'll just create a 32x32 PNG (browsers handle this well)
  await sharp(svgBuffer)
    .resize(32, 32, {
      fit: 'contain',
      background: { r: 255, g: 255, b: 255, alpha: 1 }
    })
    .png()
    .toFile(path.join(PUBLIC_DIR, 'favicon.png'));
  console.log(`  ✓ favicon.png (32x32)`);

  console.log('\n✅ PNG icons generated successfully!');
  console.log('\n⚠️  Note: ICO and ICNS files require additional tools.');
  console.log('   For Windows .ico: Use an online converter or install ImageMagick');
  console.log('   For macOS .icns: Use iconutil on macOS');
  console.log('\n   You can convert icon.png (512x512) to these formats.');
}

// For ICO generation, we'll create multiple PNG sizes that can be combined
async function generateIcoSourcePngs() {
  const svgBuffer = fs.readFileSync(SVG_SOURCE);
  const icoSizes = [16, 32, 48, 64, 128, 256];

  console.log('\nGenerating ICO source PNGs...');

  const icoDir = path.join(ICONS_DIR, 'ico-sources');
  if (!fs.existsSync(icoDir)) {
    fs.mkdirSync(icoDir, { recursive: true });
  }

  for (const size of icoSizes) {
    const outputPath = path.join(icoDir, `icon-${size}.png`);
    await sharp(svgBuffer)
      .resize(size, size, {
        fit: 'contain',
        background: { r: 255, g: 255, b: 255, alpha: 1 }
      })
      .png()
      .toFile(outputPath);
    console.log(`  ✓ icon-${size}.png`);
  }

  console.log('\n📁 ICO source PNGs saved to:', icoDir);
  console.log('   Use these with an ICO converter to create icon.ico');
}

async function main() {
  try {
    await generatePngIcons();
    await generateIcoSourcePngs();

    console.log('\n🎉 Icon generation complete!');
    console.log('\nNext steps:');
    console.log('1. Convert ico-sources/* to icon.ico using:');
    console.log('   - Online: https://icoconvert.com/ or https://convertico.com/');
    console.log('   - ImageMagick: magick convert ico-sources/*.png icon.ico');
    console.log('2. For macOS, convert on a Mac using iconutil');
  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

main();
