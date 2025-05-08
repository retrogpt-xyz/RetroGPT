// src/LatexRenderer.tsx
import React from 'react';
import katex from 'katex';
import 'katex/dist/katex.min.css'; // Import KaTeX CSS

interface LatexRendererProps {
  text: string;
}

const LatexRenderer: React.FC<LatexRendererProps> = ({ text }) => {
  // Regex to find LaTeX blocks.
  // This regex looks for:
  // 1. $...$ (inline math)
  // 2. $$...$$ (display math)
  // 3. \[...\] (display math, LaTeX standard)
  // 4. \(...\) (inline math, LaTeX standard)
  // The 'g' flag finds all occurrences.
  // The 's' flag (dotall) is not directly supported in all JS environments for lookbehinds,
  // so we use [\s\S]*? to match any character including newlines, non-greedily.
  // We need to be careful with single $ to avoid false positives in regular text with currency.
  const latexRegex = /(\$\$[\s\S]*?\$\$|\$[^\$\n]+?\$|\\\[[\s\S]*?\\\]|\\\([\s\S]*?\\\))/g;

  // Split the text by the LaTeX blocks, keeping the delimiters.
  // The filter(part => part) removes empty strings that can result from splitting.
  const parts = text.split(latexRegex).filter(part => part);

  return (
    <>
      {parts.map((part, index) => {
        let isLatex = false;
        let displayMode = false;
        let latexContent = part;

        if (part.startsWith('$$') && part.endsWith('$$')) {
          isLatex = true;
          displayMode = true;
          latexContent = part.substring(2, part.length - 2); // Remove $$
        } else if (part.startsWith('$') && part.endsWith('$') && part.length > 2) {
          isLatex = true;
          displayMode = false; // Single $ is for inline math
          latexContent = part.substring(1, part.length - 1); // Remove $
        } else if (part.startsWith('\\[') && part.endsWith('\\]')) {
          isLatex = true;
          displayMode = true;
          latexContent = part.substring(2, part.length - 2); // Remove \[ and \]
        } else if (part.startsWith('\\(') && part.endsWith('\\)')) {
          isLatex = true;
          displayMode = false;
          latexContent = part.substring(2, part.length - 2); // Remove \( and \)
        }

        if (isLatex) {
          try {
            const html = katex.renderToString(latexContent, {
              throwOnError: false, // Don't crash the app on bad LaTeX
              displayMode: displayMode,
              output: 'htmlAndMathml', // Good for accessibility
              // You can add macros here if needed:
              // macros: {"\\RR": "\\mathbb{R}"}
            });
            // KaTeX generates HTML, so we need to render it dangerously.
            // This is generally safe as KaTeX output is controlled.
            return <span key={index} dangerouslySetInnerHTML={{ __html: html }} />;
          } catch (e) {
            console.error('KaTeX rendering error:', e);
            // Fallback: render the original LaTeX string with an error indicator
            return <span key={index} style={{ color: 'red' }}>{part} (Error)</span>;
          }
        } else {
          // If it's not a LaTeX part, render it as plain text.
          // Using React.Fragment to avoid adding extra DOM elements unless necessary.
          return <React.Fragment key={index}>{part}</React.Fragment>;
        }
      })}
    </>
  );
};

export default LatexRenderer;
