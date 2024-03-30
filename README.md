# `code-similarity`

> [!NOTE]
> This is just a POC implementation! Sometimes textual similarity is 100%
> dispite having differences this seems to be an issue related to simhash-ing.
> One alternative is to use the Vector directly for calculating similarity.

This is a POC implementation of the paper
[`Code Similarity Detection Using AST and Textual Information`].\
Do note that this deviates a little from the original paper to be a little
efficient for on demand computation of similarity.

This stemmed from the need to efficiently find the difference ratio in un-bundled
modules from something like a webpack chunk of the current and previous
iteration of said chunk for the sake of datamining™.

The implementation works based on xxhash (xxh3_128) and a specialized simhash
with a bit of AST parsing.
Similar to the original paper the final difference is a weighted calculation
between the textual and AST similarity.

The following pseudo-code outlines how the similarity is calculated

```plaintext
algorithm simhash_vector is
      S := lenght of word
      H := xxh3_128_with_seed(word, S)

      for R in 0 to 128
          B := (H >> R) & 1

          if B == 1
              saturatingly increment vector[R] by 1
          else
              saturatingly decrement vector[R] by 1

algorithm simhash_hash is
    H := 0u128

    for shift in 0 to 128
        if vector[shift] > 0
            H |= 1 << shift

    return H

algorithm recurse_ast is
    simhash_vector(vector, get_label(node))

    for each child in node
        recurse_ast(vector, child)

algorithm similarity_params is
    Tᵥ := [0; 128]

    for each whitespace separated word in source
        simhash_vector(Tᵥ, word)

    Tₕ := simhash_hash(Tᵥ)

    Aᵥ := [0; 128]

    root_node := parse_source_ast(source)
    recurse_ast(Aᵥ, root_node)

    Aₕ := simhash_hash(Aᵥ)

    return (Tₕ, Aₕ)

algorithm hamming_similarity is
    H := count_ones(a ^ b)
    S := 1.0 - (H / type_bits_size(H))

    return S

algorithm similarity is
    Wₐ := clamp(weight, 0.0, 1.0)
    Wₜ := 1.0 - Wₐ

    Sₜ := hamming_similarity(a.text, b.text)
    Sₐ := hamming_similarity(a.ast, b.ast)

    S := (Sₜ * Wₜ) + (Sₐ * Wₐ)

    return S
```

[`Code Similarity Detection Using AST and Textual Information`]: https://www.ijpe-online.com/EN/10.23940/ijpe.19.10.p14.26832691
