use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum IconMask {
    Moon,
    Sun,
    Rust,
    Coffee,
    Keyboard,
    Git,
    Robot,
    Share,
    Copyright,
    LinkedIn,
    Mastodon,
    GitHub,
    CopyToClipboard,
    Question,
    Lego,
}

impl IconMask {
    pub const fn viewbox(&self) -> (f32, f32, f32, f32) {
        match self {
            IconMask::Moon => (0., 0., 512., 512.),
            IconMask::Sun => (0., 0., 512., 512.),
            IconMask::Rust => (0., 0., 512., 512.),
            IconMask::Coffee => (0., 0., 640., 512.),
            IconMask::Keyboard => (0., 0., 576., 512.),
            IconMask::Git => (0., 0., 448., 512.),
            IconMask::Robot => (0., 0., 640., 512.),
            IconMask::Share => (0., 0., 16., 16.),
            IconMask::Copyright => (0., 0., 512., 512.),
            IconMask::LinkedIn => (0., 0., 448., 512.),
            IconMask::Mastodon => (0., 0., 448., 512.),
            IconMask::GitHub => (0., 0., 512., 512.),
            IconMask::CopyToClipboard => (0., 0., 384., 512.),
            IconMask::Question => (0., 0., 512., 512.),
            IconMask::Lego => (0., 0., 1024., 1024.),
        }
    }

    pub fn aspect_ratio(icon: IconMask) -> f32 {
        let (x1, y1, x2, y2) = icon.viewbox();
        (x2 - x1) / (y2 - y1)
    }

    pub const fn data(&self) -> &'static str {
        match self {
            IconMask::Moon => {
                r#"M32 256c0-123.8 100.3-224 223.8-224c11.36 0 29.7 1.668 40.9 3.746c9.616 1.777 11.75 14.63 3.279 19.44C245 86.5 211.2 144.6 211.2 207.8c0 109.7 99.71 193 208.3 172.3c9.561-1.805 16.28 9.324 10.11 16.95C387.9 448.6 324.8 480 255.8 480C132.1 480 32 379.6 32 256z"#
            }
            IconMask::Sun => {
                r#"M361.5 1.2c5 2.1 8.6 6.6 9.6 11.9L391 121l107.9 19.8c5.3 1 9.8 4.6 11.9 9.6s1.5 10.7-1.6 15.2L446.9 256l62.3 90.3c3.1 4.5 3.7 10.2 1.6 15.2s-6.6 8.6-11.9 9.6L391 391 371.1 498.9c-1 5.3-4.6 9.8-9.6 11.9s-10.7 1.5-15.2-1.6L256 446.9l-90.3 62.3c-4.5 3.1-10.2 3.7-15.2 1.6s-8.6-6.6-9.6-11.9L121 391 13.1 371.1c-5.3-1-9.8-4.6-11.9-9.6s-1.5-10.7 1.6-15.2L65.1 256 2.8 165.7c-3.1-4.5-3.7-10.2-1.6-15.2s6.6-8.6 11.9-9.6L121 121 140.9 13.1c1-5.3 4.6-9.8 9.6-11.9s10.7-1.5 15.2 1.6L256 65.1 346.3 2.8c4.5-3.1 10.2-3.7 15.2-1.6zM352 256c0 53-43 96-96 96s-96-43-96-96s43-96 96-96s96 43 96 96zm32 0c0-70.7-57.3-128-128-128s-128 57.3-128 128s57.3 128 128 128s128-57.3 128-128z"#
            }
            IconMask::Rust => {
                r#"M508.52,249.75,486.7,236.24c-.17-2-.34-3.93-.55-5.88l18.72-17.5a7.35,7.35,0,0,0-2.44-12.25l-24-9c-.54-1.88-1.08-3.78-1.67-5.64l15-20.83a7.35,7.35,0,0,0-4.79-11.54l-25.42-4.15c-.9-1.73-1.79-3.45-2.73-5.15l10.68-23.42a7.35,7.35,0,0,0-6.95-10.39l-25.82.91q-1.79-2.22-3.61-4.4L439,81.84A7.36,7.36,0,0,0,430.16,73L405,78.93q-2.17-1.83-4.4-3.61l.91-25.82a7.35,7.35,0,0,0-10.39-7L367.7,53.23c-1.7-.94-3.43-1.84-5.15-2.73L358.4,25.08a7.35,7.35,0,0,0-11.54-4.79L326,35.26c-1.86-.59-3.75-1.13-5.64-1.67l-9-24a7.35,7.35,0,0,0-12.25-2.44l-17.5,18.72c-1.95-.21-3.91-.38-5.88-.55L262.25,3.48a7.35,7.35,0,0,0-12.5,0L236.24,25.3c-2,.17-3.93.34-5.88.55L212.86,7.13a7.35,7.35,0,0,0-12.25,2.44l-9,24c-1.89.55-3.79,1.08-5.66,1.68l-20.82-15a7.35,7.35,0,0,0-11.54,4.79l-4.15,25.41c-1.73.9-3.45,1.79-5.16,2.73L120.88,42.55a7.35,7.35,0,0,0-10.39,7l.92,25.81c-1.49,1.19-3,2.39-4.42,3.61L81.84,73A7.36,7.36,0,0,0,73,81.84L78.93,107c-1.23,1.45-2.43,2.93-3.62,4.41l-25.81-.91a7.42,7.42,0,0,0-6.37,3.26,7.35,7.35,0,0,0-.57,7.13l10.66,23.41c-.94,1.7-1.83,3.43-2.73,5.16L25.08,153.6a7.35,7.35,0,0,0-4.79,11.54l15,20.82c-.59,1.87-1.13,3.77-1.68,5.66l-24,9a7.35,7.35,0,0,0-2.44,12.25l18.72,17.5c-.21,1.95-.38,3.91-.55,5.88L3.48,249.75a7.35,7.35,0,0,0,0,12.5L25.3,275.76c.17,2,.34,3.92.55,5.87L7.13,299.13a7.35,7.35,0,0,0,2.44,12.25l24,9c.55,1.89,1.08,3.78,1.68,5.65l-15,20.83a7.35,7.35,0,0,0,4.79,11.54l25.42,4.15c.9,1.72,1.79,3.45,2.73,5.14L42.56,391.12a7.35,7.35,0,0,0,.57,7.13,7.13,7.13,0,0,0,6.37,3.26l25.83-.91q1.77,2.22,3.6,4.4L73,430.16A7.36,7.36,0,0,0,81.84,439L107,433.07q2.18,1.83,4.41,3.61l-.92,25.82a7.35,7.35,0,0,0,10.39,6.95l23.43-10.68c1.69.94,3.42,1.83,5.14,2.73l4.15,25.42a7.34,7.34,0,0,0,11.54,4.78l20.83-15c1.86.6,3.76,1.13,5.65,1.68l9,24a7.36,7.36,0,0,0,12.25,2.44l17.5-18.72c1.95.21,3.92.38,5.88.55l13.51,21.82a7.35,7.35,0,0,0,12.5,0l13.51-21.82c2-.17,3.93-.34,5.88-.56l17.5,18.73a7.36,7.36,0,0,0,12.25-2.44l9-24c1.89-.55,3.78-1.08,5.65-1.68l20.82,15a7.34,7.34,0,0,0,11.54-4.78l4.15-25.42c1.72-.9,3.45-1.79,5.15-2.73l23.42,10.68a7.35,7.35,0,0,0,10.39-6.95l-.91-25.82q2.22-1.79,4.4-3.61L430.16,439a7.36,7.36,0,0,0,8.84-8.84L433.07,405q1.83-2.17,3.61-4.4l25.82.91a7.23,7.23,0,0,0,6.37-3.26,7.35,7.35,0,0,0,.58-7.13L458.77,367.7c.94-1.7,1.83-3.43,2.73-5.15l25.42-4.15a7.35,7.35,0,0,0,4.79-11.54l-15-20.83c.59-1.87,1.13-3.76,1.67-5.65l24-9a7.35,7.35,0,0,0,2.44-12.25l-18.72-17.5c.21-1.95.38-3.91.55-5.87l21.82-13.51a7.35,7.35,0,0,0,0-12.5Zm-151,129.08A13.91,13.91,0,0,0,341,389.51l-7.64,35.67A187.51,187.51,0,0,1,177,424.44l-7.64-35.66a13.87,13.87,0,0,0-16.46-10.68l-31.51,6.76a187.38,187.38,0,0,1-16.26-19.21H258.3c1.72,0,2.89-.29,2.89-1.91V309.55c0-1.57-1.17-1.91-2.89-1.91H213.47l.05-34.35H262c4.41,0,23.66,1.28,29.79,25.87,1.91,7.55,6.17,32.14,9.06,40,2.89,8.82,14.6,26.46,27.1,26.46H407a187.3,187.3,0,0,1-17.34,20.09Zm25.77,34.49A15.24,15.24,0,1,1,368,398.08h.44A15.23,15.23,0,0,1,383.24,413.32Zm-225.62-.68a15.24,15.24,0,1,1-15.25-15.25h.45A15.25,15.25,0,0,1,157.62,412.64ZM69.57,234.15l32.83-14.6a13.88,13.88,0,0,0,7.06-18.33L102.69,186h26.56V305.73H75.65A187.65,187.65,0,0,1,69.57,234.15ZM58.31,198.09a15.24,15.24,0,0,1,15.23-15.25H74a15.24,15.24,0,1,1-15.67,15.24Zm155.16,24.49.05-35.32h63.26c3.28,0,23.07,3.77,23.07,18.62,0,12.29-15.19,16.7-27.68,16.7ZM399,306.71c-9.8,1.13-20.63-4.12-22-10.09-5.78-32.49-15.39-39.4-30.57-51.4,18.86-11.95,38.46-29.64,38.46-53.26,0-25.52-17.49-41.59-29.4-49.48-16.76-11-35.28-13.23-40.27-13.23H116.32A187.49,187.49,0,0,1,221.21,70.06l23.47,24.6a13.82,13.82,0,0,0,19.6.44l26.26-25a187.51,187.51,0,0,1,128.37,91.43l-18,40.57A14,14,0,0,0,408,220.43l34.59,15.33a187.12,187.12,0,0,1,.4,32.54H423.71c-1.91,0-2.69,1.27-2.69,3.13v8.82C421,301,409.31,305.58,399,306.71ZM240,60.21A15.24,15.24,0,0,1,255.21,45h.45A15.24,15.24,0,1,1,240,60.21ZM436.84,214a15.24,15.24,0,1,1,0-30.48h.44a15.24,15.24,0,0,1-.44,30.48Z"#
            }
            IconMask::Coffee => {
                r#"M512 32H120c-13.25 0-24 10.75-24 24L96.01 288c0 53 43 96 96 96h192C437 384 480 341 480 288h32c70.63 0 128-57.38 128-128S582.6 32 512 32zM512 224h-32V96h32c35.25 0 64 28.75 64 64S547.3 224 512 224zM560 416h-544C7.164 416 0 423.2 0 432C0 458.5 21.49 480 48 480h480c26.51 0 48-21.49 48-48C576 423.2 568.8 416 560 416z"#
            }
            IconMask::Keyboard => {
                r#"M512 448H64c-35.35 0-64-28.65-64-64V128c0-35.35 28.65-64 64-64h448c35.35 0 64 28.65 64 64v256C576 419.3 547.3 448 512 448zM128 180v-40C128 133.4 122.6 128 116 128h-40C69.38 128 64 133.4 64 140v40C64 186.6 69.38 192 76 192h40C122.6 192 128 186.6 128 180zM224 180v-40C224 133.4 218.6 128 212 128h-40C165.4 128 160 133.4 160 140v40C160 186.6 165.4 192 172 192h40C218.6 192 224 186.6 224 180zM320 180v-40C320 133.4 314.6 128 308 128h-40C261.4 128 256 133.4 256 140v40C256 186.6 261.4 192 268 192h40C314.6 192 320 186.6 320 180zM416 180v-40C416 133.4 410.6 128 404 128h-40C357.4 128 352 133.4 352 140v40C352 186.6 357.4 192 364 192h40C410.6 192 416 186.6 416 180zM512 180v-40C512 133.4 506.6 128 500 128h-40C453.4 128 448 133.4 448 140v40C448 186.6 453.4 192 460 192h40C506.6 192 512 186.6 512 180zM128 276v-40C128 229.4 122.6 224 116 224h-40C69.38 224 64 229.4 64 236v40C64 282.6 69.38 288 76 288h40C122.6 288 128 282.6 128 276zM224 276v-40C224 229.4 218.6 224 212 224h-40C165.4 224 160 229.4 160 236v40C160 282.6 165.4 288 172 288h40C218.6 288 224 282.6 224 276zM320 276v-40C320 229.4 314.6 224 308 224h-40C261.4 224 256 229.4 256 236v40C256 282.6 261.4 288 268 288h40C314.6 288 320 282.6 320 276zM416 276v-40C416 229.4 410.6 224 404 224h-40C357.4 224 352 229.4 352 236v40C352 282.6 357.4 288 364 288h40C410.6 288 416 282.6 416 276zM512 276v-40C512 229.4 506.6 224 500 224h-40C453.4 224 448 229.4 448 236v40C448 282.6 453.4 288 460 288h40C506.6 288 512 282.6 512 276zM128 372v-40C128 325.4 122.6 320 116 320h-40C69.38 320 64 325.4 64 332v40C64 378.6 69.38 384 76 384h40C122.6 384 128 378.6 128 372zM416 372v-40C416 325.4 410.6 320 404 320h-232C165.4 320 160 325.4 160 332v40C160 378.6 165.4 384 172 384h232C410.6 384 416 378.6 416 372zM512 372v-40C512 325.4 506.6 320 500 320h-40C453.4 320 448 325.4 448 332v40C448 378.6 453.4 384 460 384h40C506.6 384 512 378.6 512 372z"#
            }
            IconMask::Git => {
                r#"M80 104c13.3 0 24-10.7 24-24s-10.7-24-24-24S56 66.7 56 80s10.7 24 24 24zm80-24c0 32.8-19.7 61-48 73.3v87.8c18.8-10.9 40.7-17.1 64-17.1h96c35.3 0 64-28.7 64-64v-6.7C307.7 141 288 112.8 288 80c0-44.2 35.8-80 80-80s80 35.8 80 80c0 32.8-19.7 61-48 73.3V160c0 70.7-57.3 128-128 128H176c-35.3 0-64 28.7-64 64v6.7c28.3 12.3 48 40.5 48 73.3c0 44.2-35.8 80-80 80s-80-35.8-80-80c0-32.8 19.7-61 48-73.3V352 153.3C19.7 141 0 112.8 0 80C0 35.8 35.8 0 80 0s80 35.8 80 80zm232 0c0-13.3-10.7-24-24-24s-24 10.7-24 24s10.7 24 24 24s24-10.7 24-24zM80 456c13.3 0 24-10.7 24-24s-10.7-24-24-24s-24 10.7-24 24s10.7 24 24 24z"#
            }
            IconMask::Robot => {
                r#"M320 0c17.7 0 32 14.3 32 32V96H480c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H160c-35.3 0-64-28.7-64-64V160c0-35.3 28.7-64 64-64H288V32c0-17.7 14.3-32 32-32zM208 384c-8.8 0-16 7.2-16 16s7.2 16 16 16h32c8.8 0 16-7.2 16-16s-7.2-16-16-16H208zm96 0c-8.8 0-16 7.2-16 16s7.2 16 16 16h32c8.8 0 16-7.2 16-16s-7.2-16-16-16H304zm96 0c-8.8 0-16 7.2-16 16s7.2 16 16 16h32c8.8 0 16-7.2 16-16s-7.2-16-16-16H400zM264 256c0-22.1-17.9-40-40-40s-40 17.9-40 40s17.9 40 40 40s40-17.9 40-40zm152 40c22.1 0 40-17.9 40-40s-17.9-40-40-40s-40 17.9-40 40s17.9 40 40 40zM48 224H64V416H48c-26.5 0-48-21.5-48-48V272c0-26.5 21.5-48 48-48zm544 0c26.5 0 48 21.5 48 48v96c0 26.5-21.5 48-48 48H576V224h16z"#
            }
            IconMask::Share => {
                r#"M10.99 13.99h-9v-9h4.76l2-2H.99c-.55 0-1 .45-1 1v11c0 .55.45 1 1 1h11c.55 0 1-.45 1-1V7.24l-2 2v4.75zm4-14h-5c-.55 0-1 .45-1 1s.45 1 1 1h2.59L7.29 7.28a1 1 0 00-.3.71 1.003 1.003 0 001.71.71l5.29-5.29V6c0 .55.45 1 1 1s1-.45 1-1V1c0-.56-.45-1.01-1-1.01z"#
            }
            IconMask::Copyright => {
                r#"M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM199.2 312.6c14.94 15.06 34.8 23.38 55.89 23.38c.0313 0 0 0 0 0c21.06 0 40.92-8.312 55.83-23.38c9.375-9.375 24.53-9.469 33.97-.1562c9.406 9.344 9.469 24.53 .1562 33.97c-24 24.22-55.95 37.56-89.95 37.56c0 0 .0313 0 0 0c-33.97 0-65.95-13.34-89.95-37.56c-49.44-49.88-49.44-131 0-180.9c24-24.22 55.98-37.56 89.95-37.56c.0313 0 0 0 0 0c34 0 65.95 13.34 89.95 37.56c9.312 9.438 9.25 24.62-.1562 33.97c-9.438 9.344-24.59 9.188-33.97-.1562c-14.91-15.06-34.77-23.38-55.83-23.38c0 0 .0313 0 0 0c-21.09 0-40.95 8.312-55.89 23.38C168.3 230.6 168.3 281.4 199.2 312.6z"#
            }
            IconMask::LinkedIn => {
                r#"M100.28 448H7.4V148.9h92.88zM53.79 108.1C24.09 108.1 0 83.5 0 53.8a53.79 53.79 0 0 1 107.58 0c0 29.7-24.1 54.3-53.79 54.3zM447.9 448h-92.68V302.4c0-34.7-.7-79.2-48.29-79.2-48.29 0-55.69 37.7-55.69 76.7V448h-92.78V148.9h89.08v40.8h1.3c12.4-23.5 42.69-48.3 87.88-48.3 94 0 111.28 61.9 111.28 142.3V448z"#
            }
            IconMask::Mastodon => {
                r#"M433 179.11c0-97.2-63.71-125.7-63.71-125.7-62.52-28.7-228.56-28.4-290.48 0 0 0-63.72 28.5-63.72 125.7 0 115.7-6.6 259.4 105.63 289.1 40.51 10.7 75.32 13 103.33 11.4 50.81-2.8 79.32-18.1 79.32-18.1l-1.7-36.9s-36.31 11.4-77.12 10.1c-40.41-1.4-83-4.4-89.63-54a102.54 102.54 0 0 1-.9-13.9c85.63 20.9 158.65 9.1 178.75 6.7 56.12-6.7 105-41.3 111.23-72.9 9.8-49.8 9-121.5 9-121.5zm-75.12 125.2h-46.63v-114.2c0-49.7-64-51.6-64 6.9v62.5h-46.33V197c0-58.5-64-56.6-64-6.9v114.2H90.19c0-122.1-5.2-147.9 18.41-175 25.9-28.9 79.82-30.8 103.83 6.1l11.6 19.5 11.6-19.5c24.11-37.1 78.12-34.8 103.83-6.1 23.71 27.3 18.4 53 18.4 175z"#
            }
            IconMask::GitHub => {
                r#"M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"#
            }
            IconMask::CopyToClipboard => {
                r#"M192 0c-41.8 0-77.4 26.7-90.5 64H64C28.7 64 0 92.7 0 128V448c0 35.3 28.7 64 64 64H320c35.3 0 64-28.7 64-64V128c0-35.3-28.7-64-64-64H282.5C269.4 26.7 233.8 0 192 0zm0 64a32 32 0 1 1 0 64 32 32 0 1 1 0-64zM112 192H272c8.8 0 16 7.2 16 16s-7.2 16-16 16H112c-8.8 0-16-7.2-16-16s7.2-16 16-16z"#
            }
            IconMask::Question => {
                r#"M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM169.8 165.3c7.9-22.3 29.1-37.3 52.8-37.3h58.3c34.9 0 63.1 28.3 63.1 63.1c0 22.6-12.1 43.5-31.7 54.8L280 264.4c-.2 13-10.9 23.6-24 23.6c-13.3 0-24-10.7-24-24V250.5c0-8.6 4.6-16.5 12.1-20.8l44.3-25.4c4.7-2.7 7.6-7.7 7.6-13.1c0-8.4-6.8-15.1-15.1-15.1H222.6c-3.4 0-6.4 2.1-7.5 5.3l-.4 1.2c-4.4 12.5-18.2 19-30.6 14.6s-19-18.2-14.6-30.6l.4-1.2zM224 352a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z"#
            }
            IconMask::Lego => {
                r#"M768 160h-32V64a32 32 0 0 0-32-32H320a32 32 0 0 0-32 32v96h-32A192 192 0 0 0 64 352v352a192 192 0 0 0 192 192v64a32 32 0 0 0 32 32h448a32 32 0 0 0 32-32v-64a192 192 0 0 0 192-192V352a192 192 0 0 0-192-192zM296.32 431.68a64 64 0 1 1 64 64 64 64 0 0 1-64-64zM684.16 640A217.92 217.92 0 0 1 512 716.8 217.92 217.92 0 0 1 339.84 640a35.84 35.84 0 1 1 56.32-44.16A148.16 148.16 0 0 0 512 645.44a148.16 148.16 0 0 0 115.84-50.24 35.84 35.84 0 0 1 56.32 44.8zM664 496.32a64 64 0 1 1 64-64 64 64 0 0 1-64 63.04z"#
            }
        }
    }
}

impl fmt::Display for IconMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, width, height) = self.viewbox();
        write!(
            f,
            "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='{} {} {} {}'%3E%3Cpath d='{}'/%3E%3C/svg%3E",
            x,
            y,
            width,
            height,
            self.data()
        )
    }
}
