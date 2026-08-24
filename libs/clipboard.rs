#[must_use]
pub fn toast_script(content: &str, message: &str) -> String {
    format!(
        "
        (() => {{
            const text = {content:?};
            navigator.clipboard.writeText(text).catch(() => {message:?});
            const existing = document.querySelector('.copy-toast');
            if (existing) {{
                existing.remove();
            }}
            const toast = document.createElement('div');
            toast.className = 'copy-toast';
            toast.setAttribute('role', 'status');
            toast.setAttribute('aria-live', 'polite');
            toast.textContent = {message:?};
            Object.assign(toast.style, {{
                position: 'fixed',
                bottom: '1.5rem',
                right: '1.5rem',
                padding: '0.35rem 0.75rem',
                borderRadius: '9999px',
                fontSize: '0.75rem',
                fontFamily: 'inherit',
                background: 'rgba(0, 0, 0, 0.75)',
                color: 'var(--color-base-light, #f2eeeb)',
                boxShadow: '0 4px 14px rgba(0, 0, 0, 0.25)',
                zIndex: '9999',
                opacity: '0',
                transition: 'opacity 150ms ease-in-out',
                pointerEvents: 'none',
            }});
            const body = document.body;
            if (!body) {{
                return;
            }}
            body.appendChild(toast);
            requestAnimationFrame(() => {{
                toast.style.opacity = '1';
            }});
            setTimeout(() => {{
                toast.style.opacity = '0';
                setTimeout(() => toast.remove(), 200);
            }}, 1600);
        }})();
        "
    )
}
