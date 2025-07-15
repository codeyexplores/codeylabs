// Integrated with rust backend

class News {
  final String title;
  final String description;
  final String link;
  final String published;
  final String? summary;

  News({
    required this.title,
    required this.description,
    required this.link,
    required this.published,
    this.summary,
  });

  factory News.fromJson(Map<String, dynamic> json) {
    return News(
      title: json['title'] ?? 'No title',
      description: json['description'] ?? 'No description',
      link: json['link'] ?? '',
      published: json['published'],
      summary: json['summary'], // Optional
    );
  }
}



// // Flutter - only
// class News {
//   final String title;
//   final String body;
//   final String url;  // Add this

//   News({
//     required this.title,
//     required this.body,
//     required this.url,
//   });

//   factory News.fromJson(Map<String, dynamic> json) {
//     return News(
//       title: json['title'] ?? 'No title',
//       body: json['description'] ?? 'No description',
//       url: json['link'] ?? '', // or 'url' depending on API
//     );
//   }
// }