# USV Telemetry System

A modern, real-time telemetry dashboard for Unmanned Surface Vehicles (USV). This system provides comprehensive monitoring capabilities including GPS tracking, camera feeds, and real-time sensor data visualization with an intuitive web interface.

![dashboard](URL-ou-chemin-de-l-image)


## Features

### 🎯 Real-time Monitoring
- **Camera Feed**: Large main panel with integrated camera view and pan/tilt controls
- **Live GPS Tracking**: Compact interactive map for real-time position monitoring
- **Telemetry Data**: Comprehensive sensor data display including battery, engine, and environmental sensors
- **System Status**: Compact grid layout showing all system indicators at a glance

### 🎨 Modern UI/UX
- **Responsive Design**: Fully responsive interface that works on desktop, tablet, and mobile devices
- **Glass Morphism**: Modern design with translucent panels and blur effects
- **Real-time Updates**: Live data updates with smooth animations and transitions
- **Professional Dashboard**: Clean, intuitive interface designed for mission-critical operations

### 📊 Data Visualization
- **Interactive Map**: Compact visual representation of USV position and heading in top-right panel
- **Progress Indicators**: Battery levels, fuel status, and system health metrics
- **Status Grid**: 2-column layout showing all system indicators without scrolling
- **Telemetry Metrics**: Detailed sensor readings and operational parameters in bottom-right panel

## Technical Architecture

### Built With
- **HTML5**: Modern web standards with semantic markup
- **JavaScript**: Real-time data simulation and interactive features
- **CSS3**: Advanced styling with CSS Grid, Flexbox, and glass morphism design
- **Responsive Design**: Mobile-first approach with flexible layouts

### Dashboard Layout
The dashboard features a 4-panel layout optimized for operational efficiency:

1. **Camera Feed** (Main panel - left side): Large video stream with control buttons
2. **System Status** (Bottom-left): 2x3 grid showing all system indicators
3. **USV Tracking Map** (Top-right): Compact GPS visualization with position tracking
4. **Telemetry Data** (Bottom-right): Detailed sensor readings and metrics

### File Structure
```
usv_2/
├── index.html           # Main dashboard interface
├── assets/styles.css    # Modern CSS with glass morphism design
├── serve.py            # Development server with CORS support
├── README.md           # Project documentation
├── DEMO.md             # Detailed demo guide
└── src/                # Rust source code (alternative implementation)
```

## Data Models

### Telemetry Data Structure
The system tracks comprehensive telemetry data including:

- **Position**: GPS coordinates (latitude, longitude, altitude)
- **Velocity**: Speed, heading, and vertical speed
- **Battery**: Level, voltage, current, temperature, and charging status
- **Engine**: Throttle, RPM, temperature, fuel level, and operational status
- **Sensors**: Compass heading, orientation (pitch/roll/yaw), environmental data
- **Communication**: Signal strength, network type, connection status

## Getting Started

### Prerequisites
- Python 3 (for the development server)
- Modern web browser (Chrome, Firefox, Safari, Edge)
- No additional installations required

### Quick Start

1. **Clone or download the project**
   ```bash
   # If using git
   git clone <repository-url>
   cd usv_2
   ```

2. **Start the development server**
   ```bash
   python3 serve.py
   ```
   The server will automatically find an available port and display the URL.

3. **Open your browser**
   Navigate to the URL shown (typically `http://localhost:3000`)

That's it! The dashboard will load immediately with live telemetry simulation.

### Alternative Servers

If Python is not available, you can use any HTTP server:

```bash
# Using Node.js (if installed)
npx http-server . -p 8000 --cors

# Using PHP (if installed)
php -S localhost:8000

# Using any static file server
```

### Production Deployment

For production deployment, simply serve the files using any web server:
- Apache
- Nginx  
- Any CDN or static hosting service

No build process required - the dashboard is ready to use as-is.

## Usage

### Dashboard Layout
The dashboard is organized into four main sections:

1. **Camera Feed** (Main panel - left): Large live camera stream with control buttons
2. **System Status** (Bottom-left): Compact 2x3 grid showing all system indicators
3. **USV Tracking Map** (Top-right): Interactive map showing USV position and trajectory
4. **Telemetry Panel** (Bottom-right): Real-time sensor data and metrics

### Real-time Data
The system automatically generates mock telemetry data for demonstration purposes. In a production environment, this would be replaced with actual sensor integration.

### Responsive Design
The interface automatically adapts to different screen sizes:
- **Desktop**: Full dashboard layout with all panels visible
- **Tablet**: Optimized layout with reorganized panels
- **Mobile**: Single-column layout with stacked components

## Development

### Code Style
The project follows Rust best practices and modern development principles:

- **KISS**: Keep It Simple, Stupid - Simple, focused components
- **DRY**: Don't Repeat Yourself - Reusable components and utilities
- **SOLID**: Well-structured, maintainable code architecture
- **Clean Code**: Readable, well-documented, and maintainable

### Component Architecture
Each component is designed to be:
- **Self-contained**: Minimal external dependencies
- **Reusable**: Can be used in different contexts
- **Responsive**: Adapts to different screen sizes
- **Performant**: Optimized for real-time updates

### Styling Approach
- **CSS-in-Rust**: Styles are embedded within components using Dioxus style macros
- **Modern CSS**: Utilizes CSS Grid, Flexbox, and modern properties
- **Responsive Design**: Mobile-first approach with breakpoints
- **Performance**: Optimized CSS with minimal overhead

## Performance Optimization

- **WebAssembly**: Compiled to WASM for near-native performance
- **Efficient Updates**: Only re-renders components when data changes
- **Minimal Dependencies**: Carefully selected, lightweight dependencies
- **Memory Management**: Rust's ownership system ensures memory safety

## Browser Compatibility

- Chrome/Chromium 80+
- Firefox 79+
- Safari 14+
- Edge 80+

## Troubleshooting

### Common Issues

**Problem**: Browser shows blank page
- **Solution**: Make sure you're using an HTTP server (not opening `index.html` directly) and check browser console for errors.

### Runtime Issues

**Problem**: Styles not loading correctly
- **Solution**: Check that `assets/styles.css` is present and the server has proper MIME type support.

**Problem**: Real-time updates not working
- **Solution**: Check browser console for JavaScript errors. The dashboard uses modern JavaScript features.

### Development Server Issues

**Problem**: "Address already in use" error
- **Solution**: The server automatically finds an available port. If issues persist, manually specify a different port or kill the process using the port.

**Problem**: CORS errors in browser console
- **Solution**: The development server includes CORS headers. Try clearing browser cache or using a different browser.

### Performance

**Problem**: Slow loading
- **Solution**: The dashboard loads instantly as it's pure HTML/CSS/JS. Check your network connection and browser performance.

**Problem**: High CPU/memory usage
- **Solution**: Modern browsers handle the dashboard efficiently. Close unnecessary tabs or try a different browser.

## Security

- **Client-side Only**: Dashboard runs entirely in the browser with no server-side vulnerabilities
- **Input Validation**: All data inputs are validated and sanitized via JavaScript
- **Secure Communications**: Ready for HTTPS deployment
- **No External Dependencies**: Self-contained application with minimal external resources

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- **Web Standards Community**: For modern HTML5, CSS3, and JavaScript capabilities
- **Maritime Industry**: For inspiration and requirements gathering
- **Open Source Community**: For tools and techniques that made this project possible

---

**Built with ❤️ using modern web technologies**
