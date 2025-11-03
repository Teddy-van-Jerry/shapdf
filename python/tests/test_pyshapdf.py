"""Tests for pyshapdf Python bindings"""

import os
import tempfile
import pytest


def test_import():
    """Test that the module can be imported"""
    import pyshapdf
    assert hasattr(pyshapdf, 'render_script')


def test_render_simple_script():
    """Test rendering a simple script with a circle"""
    import pyshapdf

    script = """
    page letter
    circle 100mm 150mm 20mm color=blue
    """

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        pyshapdf.render_script(script, output_path)
        assert os.path.exists(output_path)
        assert os.path.getsize(output_path) > 0

        # Check PDF header
        with open(output_path, 'rb') as f:
            header = f.read(8)
            assert header.startswith(b'%PDF-')
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)


def test_render_multi_page():
    """Test rendering a multi-page document"""
    import pyshapdf

    script = """
    page letter
    circle 50mm 50mm 10mm color=red

    page a4
    rectangle 100mm 100mm 30mm 20mm color=green
    """

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        pyshapdf.render_script(script, output_path)
        assert os.path.exists(output_path)

        # Multi-page PDF should be larger
        assert os.path.getsize(output_path) > 500
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)


def test_render_with_rotation():
    """Test rendering shapes with rotation"""
    import pyshapdf

    script = """
    page letter
    rectangle 100mm 150mm 40mm 30mm color=blue anchor=center angle=45deg
    """

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        pyshapdf.render_script(script, output_path)
        assert os.path.exists(output_path)
        assert os.path.getsize(output_path) > 0
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)


def test_render_with_settings():
    """Test rendering with default settings"""
    import pyshapdf

    script = """
    set default_color red
    set default_width 2mm

    page letter
    line 20mm 20mm 100mm 100mm
    circle 150mm 150mm 15mm
    """

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        pyshapdf.render_script(script, output_path)
        assert os.path.exists(output_path)
        assert os.path.getsize(output_path) > 0
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)


def test_invalid_script():
    """Test that invalid scripts raise RuntimeError"""
    import pyshapdf

    script = "invalid command here"

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        with pytest.raises(RuntimeError):
            pyshapdf.render_script(script, output_path)
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)


def test_color_formats():
    """Test different color formats"""
    import pyshapdf

    script = """
    page letter
    circle 30mm 30mm 10mm color=blue
    circle 60mm 30mm 10mm color=#ff0000
    circle 90mm 30mm 10mm color=rgb(0,255,0)
    circle 120mm 30mm 10mm color=gray(0.5)
    """

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        pyshapdf.render_script(script, output_path)
        assert os.path.exists(output_path)
        assert os.path.getsize(output_path) > 0
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)


def test_units():
    """Test different unit types"""
    import pyshapdf

    script = """
    page letter
    circle 1in 1in 0.5in color=red
    circle 50mm 50mm 10mm color=green
    circle 5cm 10cm 1cm color=blue
    circle 200pt 300pt 30pt color=yellow
    """

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        pyshapdf.render_script(script, output_path)
        assert os.path.exists(output_path)
        assert os.path.getsize(output_path) > 0
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)


def test_line_caps():
    """Test different line cap types"""
    import pyshapdf

    script = """
    page letter
    line 20mm 20mm 100mm 20mm width=5mm cap=butt color=red
    line 20mm 40mm 100mm 40mm width=5mm cap=round color=green
    line 20mm 60mm 100mm 60mm width=5mm cap=square color=blue
    """

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        pyshapdf.render_script(script, output_path)
        assert os.path.exists(output_path)
        assert os.path.getsize(output_path) > 0
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)


def test_anchors():
    """Test different anchor points for rectangles"""
    import pyshapdf

    script = """
    page letter
    # Mark center point
    circle 100mm 150mm 1mm color=red

    # Rectangle with center anchor
    rectangle 100mm 150mm 40mm 30mm color=blue anchor=center
    """

    with tempfile.NamedTemporaryFile(suffix='.pdf', delete=False) as f:
        output_path = f.name

    try:
        pyshapdf.render_script(script, output_path)
        assert os.path.exists(output_path)
        assert os.path.getsize(output_path) > 0
    finally:
        if os.path.exists(output_path):
            os.remove(output_path)
